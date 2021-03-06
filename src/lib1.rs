use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;
use ndarray::*;
use ndarray_linalg::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::{self,Display,Debug,Formatter};
//use serde::{Serialize, Deserialize};
use serde::{Serialize,Deserialize};
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
use std::convert::TryInto;


// count the number of newly generated tokens which we get from 
// SentencesAsIndicesDynamicsN and
// makes map from the token indices to the contiguous index (without a possible gaps)
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratedTokensCounting {
    pub eng_renum_new_old:HashMap<Ind,Ind>,
    pub fra_renum_new_old:HashMap<Ind,Ind>,
    pub eng_renum_old_new:HashMap<Ind,Ind>,
    pub fra_renum_old_new:HashMap<Ind,Ind>,
    pub eng_number_of_tokens:Ind,
    pub fra_number_of_tokens:Ind,
}

impl GeneratedTokensCounting {
    pub fn from_sentences_as_indices_dynamics(sentences:&SentencesAsIndicesDynamicsN) -> GeneratedTokensCounting {
        let mut eng_map_new_old:HashMap<Ind,Ind> = HashMap::new();
        let mut fra_map_new_old:HashMap<Ind,Ind> = HashMap::new();
        let mut eng_map_old_new:HashMap<Ind,Ind> = HashMap::new();
        let mut fra_map_old_new:HashMap<Ind,Ind> = HashMap::new();
        let mut new_index=0;
        for (ixs,vec) in &sentences.eng_sentence_flattened_to_token_indices {
            for ind in vec.iter() {
                if !eng_map_old_new.contains_key(&ind) {
                eng_map_old_new.insert(*ind,new_index);
                new_index+=1;
                }
            }
        }
        
        for (ind1,ind2) in &eng_map_old_new {
                eng_map_new_old.insert(*ind2,*ind1);
        }

        let eng_number = eng_map_new_old.keys().len();

        let mut new_index=0;
        for (ixs,vec) in &sentences.fra_sentence_flattened_to_token_indices {
            for ind in vec.iter() {
                if !fra_map_old_new.contains_key(&ind) {
                fra_map_old_new.insert(*ind,new_index);
                new_index+=1;
                }
            }
        }
        
        for (ind1,ind2) in &fra_map_old_new {
                fra_map_new_old.insert(*ind2,*ind1);
        }

        let fra_number = fra_map_new_old.keys().len();

        

       GeneratedTokensCounting {
            eng_renum_new_old:eng_map_new_old,
            fra_renum_new_old:fra_map_new_old,
            eng_renum_old_new:eng_map_old_new,
            fra_renum_old_new:fra_map_old_new,
            eng_number_of_tokens:eng_number,
            fra_number_of_tokens:fra_number,
        }
    }
}
// we have to renumber words because of : 
// have to add some special token-words to vocab like "NoWord", "EOS", "BOS" etc...
// the tokens which are generated by tokenizer at the level of words pairs may be 
// not contiguos and have some gaps, we have to renumber in contiguous indexing to 
// control the size of vocab
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratedWordsCounting {
    pub eng_renum_new_old:HashMap<Ixx,Ixx>,
    pub fra_renum_new_old:HashMap<Ixx,Ixx>,
    pub eng_renum_old_new:HashMap<Ixx,Ixx>,
    pub fra_renum_old_new:HashMap<Ixx,Ixx>,
    pub eng_without_specials:Qxx,
    pub fra_without_specials:Qxx,
    pub special_words:Vec<String>,

}

impl GeneratedWordsCounting {
    pub fn new() -> GeneratedWordsCounting{
        GeneratedWordsCounting {
            eng_renum_new_old:HashMap::new(),   
            fra_renum_new_old:HashMap::new(),
            eng_renum_old_new:HashMap::new(),
            fra_renum_old_new:HashMap::new(),
            eng_without_specials:0,
            fra_without_specials:0,
            special_words:vec!["NoWord".to_string(), "EOS".to_string(), "BOS".to_string()],    
        }
    }   

    pub fn from_initial_vocab(&mut self, vocab:&Vocab) {
// start indexation to old words from index = special_words.len();
// first indices in new indexation are for special words
        let specials = self.special_words.len();
        let mut new_index = specials;
        for (ixx,word) in vocab.eng_index_word.iter() {
// there are now words in old vocab which correspond to first self.special_index.len() new indices
            self.eng_renum_new_old.insert(new_index,*ixx);
            self.eng_renum_old_new.insert(*ixx,new_index);
            new_index+=1;
        }

        self.eng_without_specials = new_index.try_into().unwrap();

// start indexation to old words from index = special_words.len();
// first indices in new indexation are for special words
        let mut new_index = specials;
        for (ixx,word) in vocab.fra_index_word.iter() {
// there are now words in old vocab which correspond to first self.special_index.len() new indices
            self.fra_renum_new_old.insert(new_index,*ixx);
            self.fra_renum_old_new.insert(*ixx,new_index);
            new_index+=1;
        }
        self.fra_without_specials = new_index.try_into().unwrap();
    }
}

// map word in target sentence to the list of sentences (in source) which correspond to the 
// word
#[derive(Serialize,Deserialize,Debug)]
pub struct Position {
    pub words:HashMap<Ixx,Vec<Ixs>>,
    pub max_length:usize,
    pub min_length:usize
}

impl Position {
    pub fn new() -> Position{
        Position {
            words:HashMap::new(),
            max_length:0,
            min_length:usize::MAX
        }
    }
}
// a first word in target sentence is linked with collection of sentences in source language : 
// that sentences which translate with the word as first word
#[derive(Serialize, Deserialize,Debug)]
pub struct WordsInTargetToSentences {
//position in the vector correspond to the order of words in sentences 0, 1, 2, --nd word...
//the hash map is mapping from concrete words in the positions to the vector of indices of 
//sentences which correspond to the word
//second position is the tuple: min lengs of collection, third: max length
    pub words_sentences_collections:Vec<Position>,
}

impl WordsInTargetToSentences {
    pub fn new() -> WordsInTargetToSentences {
        WordsInTargetToSentences {
            words_sentences_collections:vec![],
        }
    }
    pub fn from_array_of_words(&mut self, arr:&Array2<usize>) {
        let num_rows = arr.shape()[0];
        let num_cols = arr.shape()[1];
        let mut fst;
        for j in 0..num_cols {
        let mut map = Position::new();
            for i in 0..num_rows {
                fst = arr[[i,j]];
                if !map.words.contains_key(&fst) {
                    map.words.insert(fst,vec![i]);
                } else {
                    map.words.entry(fst).or_insert(vec![i]).push(i);
                }
            }
            self.words_sentences_collections.push(map);
        }   
    }

    pub fn max_and_min(&mut self) {
        for i in 0..self.words_sentences_collections.len() {
            let mut max = 0;
            let mut min = usize::MAX;
            self.words_sentences_collections[i].words
                .iter()
                .map(|(ixs,collection)| {
                    let size = collection.len();
                    if size > max {
                        max=size;
                    }
                    if min > size {
                        min = size;
                    }

                })
            .for_each(drop);

            self.words_sentences_collections[i].min_length=min;
            self.words_sentences_collections[i].max_length=max;
        }   
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Lengths {
//    #[serde(serialize_with = "serialize_map_a")]
    pub words:HashMap<Ixx,Vec<u8>>,
}

impl Lengths {
    pub fn new() -> Lengths{
        Lengths {
            words:HashMap::new(),
        }
    }
}


//map first words in target sentences to the list of sentence lengths (which have the word as 
//first word) , we will have a list like this [2,3,6,3,2,.....] where the numbers are the sentence
//length
//we do the same for second, third....words 
#[derive(Serialize, Deserialize,Debug)]
pub struct WordsToSentenceLengths {
    pub words_to_lengths:Vec<Lengths>,
}

impl WordsToSentenceLengths {
    pub fn new() -> WordsToSentenceLengths {
        WordsToSentenceLengths {
            words_to_lengths:Vec::new(),
        }
    }

    pub fn from_words_to_sentences(words:&WordsInTargetToSentences
                                   ,sentences:&SentencesAsIndicesDynamicsN) -> WordsToSentenceLengths {
        let mut vectr = Vec::new();
        for position in words.words_sentences_collections.iter() {
        let mut hsh = Lengths::new();
            for (word,collection) in &position.words {
                let mut coll = Vec::new();
                let mut length;
                for i in collection {
                    length = sentences.fra_words_as_indices
                        .get(i).unwrap().len();
                    coll.push(length as u8);
                }
                hsh.words.insert(word.to_owned(),coll);
            }
        vectr.push(hsh);
        }

        WordsToSentenceLengths {
            words_to_lengths:vectr,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentencesMaxLengths {
    pub target_sentence_max_len:usize,
    pub source_sentence_max_len:usize,

}

impl SentencesMaxLengths {
    pub fn from_sentences(sentences:&SentencesAsIndicesDynamicsN) 
        -> SentencesMaxLengths {
    //TODO move the max length calc to struct
    //max lengh of sentences (target and sourse) in word tokens
        let mut target_sentence_max_len = 0;
        for (ixs, vec) in &sentences.fra_words_as_indices {
            if vec.len() >= target_sentence_max_len {
                target_sentence_max_len=vec.len();
            }
        }
        let mut source_sentence_max_len = 0;
        for (ixs, vec) in &sentences.eng_words_as_indices {
            if vec.len() >= source_sentence_max_len {
                source_sentence_max_len=vec.len();
            }
        }

        SentencesMaxLengths {
            target_sentence_max_len:target_sentence_max_len,
            source_sentence_max_len:source_sentence_max_len,


        }
        
    }
}

















