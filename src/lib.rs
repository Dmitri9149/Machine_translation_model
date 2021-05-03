use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;
use ndarray::*;
use ndarray_linalg::*;
use std::fmt::{self,Display,Debug,Formatter};
//use serde::{Serialize, Deserialize};
use serde::{Serialize,Deserialize};
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
use std::convert::TryInto;



mod translation_corpus;
mod sentence_pairs;
mod word_vocabs;
mod tokens_vocab;
mod system_dynamic;
mod sentence_dynamic;

pub mod word_dynamics;

pub use crate::translation_corpus::{CorpusAsString};
pub use crate::sentence_pairs::{SentencesForTranslation
    ,SentencesAsIndices
    ,TranslationPair
    ,TranslationPairs};
pub use crate::word_vocabs::{Vocab,WordToIndexCollection};
pub use tokens_vocab::{VocabOfTokens};
pub use system_dynamic::{CandidatesForMerge
    ,CandidatesForMergeLang
    ,MostFrequentPair
    ,MostFrequentPairLang
    ,TokensAndWordsDynamics
    ,TokensAndWordsDynamicsLang
    ,Token
    ,WordsAsTokens
    ,WordsAsTokensLang
    ,SentencesAsIndicesDynamicsLang
    ,SentencesAsIndicesDynamics
};
pub use sentence_dynamic::{SentenceAsWords
    ,SentenceAsWordsLang
    ,WordsAndSentenceDynamicsLang
    ,WordsAndSentenceDynamics
    ,CandidatesForMergeLang as OtherCandidatesForMergeLang
    ,MostFrequentPairLang as OtherMostFrequentPairLang
    ,MostFrequentPair as OtherMostFrequentPair

//    ,Idiom
//    ,CandidatesForMerge
//    ,CandidatesForMergeLang
};

pub use word_dynamics::dynamics::{CandidatesForMergeN
    ,MostFrequentPairN
    ,TokensAndWordsDynamicsN
    ,TokenN
    ,WordsAsTokensN
    ,SentencesAsIndicesDynamicsN
};


// the index for sentences numbering
pub type Ixs = usize;
// indexation of tokens 
pub type Ind = usize;
// for tokens quantity
pub type Quant = u32;
// indexation of words
pub type Ixx= usize;
// for words quantity
pub type Qxx = u32;
// Lang varints 
pub enum Lang {
    Eng,
    Fra
} 
// for word length 
pub type Ixw=u16;

//the values of the hyperparams are generated by runnung bin crate renumbering ->
//   src/bin/renumbering.rs
pub static NUMBER_TOKENS_ENG:Ind = 1966;
pub static NUMBER_TOKENS_FRA:Ind = 2013;
pub static NUMBER_PAIRS:usize = 185583;
pub static NUMBER_WORDS_ENG:usize = 40000;
pub static NUMBER_WORDS_FRA:usize = 40000;
// represent special words token which designate NOWORD in some position
//TODO change to Enum struct
pub static NOWORD:usize = usize::MAX;

//static SENTENCES_NUMBER:Ixs = 190000;
// max quantity of words in a sentence
//static MAX_WORDS_IN_SENTENCE_SOURCE:Ixx = 30;
//static MAX_WORDS_IN_SENTENCE_TARGET:Ixx = 30;


pub fn replace_chars_to_char(input:&str, aa:&str, b:char) -> String {
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        if aa.contains(c) {
            output.push(b);
        } else {
            output.push(c);
        }
    }

    output
}

// separate a punctuation symbol from the list 'st' by " " from both sides 
pub fn separate_punctuation(strng:&str, st:&str) -> String{
    let mut res=strng.to_owned();
    for ch in st.chars() {
        res = res
            .replace(&ch.to_string(), &[" ", &ch.to_string()].join(""))
            .to_owned();
    }
    res
}

/*
// the function return the key with biggest value for HashMap
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}
*/

// the function return the key with biggest value
pub fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<(&K,&V)>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
//        .map(|(k, v)| (k,v))
}
// take vector of numbers find pais which are equal to pair-parameter and change the pair to new
// (number)
pub fn find_and_replace_pair(vect:&mut Vec<Ind>,pair:&(Ind,Ind),new:&Ind) {
    let size = vect.len();
    if size == 0 {
        panic!("The vector of Ind is empty !!");
    } else if size == 1 {
        return ()
    }
    let mut pointer = 0;
    let mut collector:Vec<Ind> = Vec::new();
    let mut counter = 0;
    let mut flag = false;
    while counter < size-1 {
        if (vect[counter],vect[counter+1]) == (pair.0,pair.1) {
            collector.append(&mut vect[pointer..counter].to_vec());
            collector.push(*new);
            counter+=2;
            pointer = counter;
            flag = true;

            continue;
        } 
        counter+=1;
    }

    if flag == false {
        return ()
    } else {
        collector.append(&mut vect[pointer..size].to_vec());
    }

    *vect = collector;
}

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
                    length = sentences.eng_words_as_indices
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(0,0);
    }

    #[test]
    fn find_and_replace_pair_b() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector1 = vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100];
        let mut vector2 = vec![3];
        let mut vector3 = vec![3,100,3,100,3,100,3,100,1,2,3,3,100];
        let mut vector4 = vec![3,100];

        find_and_replace_pair(&mut vector1,&pair,&new);
        find_and_replace_pair(&mut vector2,&pair,&new);
        find_and_replace_pair(&mut vector3,&pair,&new);
        find_and_replace_pair(&mut vector4,&pair,&new);

        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], vector1);
        assert_eq!(vec![3], vector2);
        assert_eq!(vec![777,777,777,777,1,2,3,777], vector3);
        assert_eq!(vec![777], vector4);


        let pair = (57,62);
        let new = 91;
        let mut vector5 = vec![71, 63, 66, 66, 73, 57, 62, 55];
        find_and_replace_pair(&mut vector5,&pair,&new);
        assert_eq!(vec![71,63,66,66,73,91,55], vector5);
       
    }

    #[test]
    #[should_panic]
    fn find_and_change_pairs_in_vector_with_panic() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector = vec![];
        find_and_replace_pair(&mut vector,&pair,&new);
    }

    #[test]
    fn find_and_replace_in_iter_mut() {

        let pair = (3,100);
        let new = 777;
        let mut map:BTreeMap<Ixx,Vec<Ind>> = BTreeMap::new();
        map.insert(0,vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100]);
        map.insert(1,vec![3]);
        map.insert(2,vec![3,100,3,100,3,100,3,100,1,2,3,3,100]);
        map.insert(3,vec![3,100]);

        for (_index,vector) in map.iter_mut() {
            find_and_replace_pair(vector,&pair,&new);
        }

        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], *map.get(&0).unwrap());
        assert_eq!(vec![3], *map.get(&1).unwrap());
        assert_eq!(vec![777,777,777,777,1,2,3,777], *map.get(&2).unwrap());
        assert_eq!(vec![777], *map.get(&3).unwrap()); 
    }
}

















