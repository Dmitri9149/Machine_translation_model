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

#[derive(Serialize, Deserialize, Debug)]
pub struct SentencesAsIndicesDynamicsN {
    pub eng_words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub eng_words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub eng_sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>,
    pub fra_words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub fra_words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub fra_sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>

}


// map word in target sentence to the list of sentences (in source) which correspond to the 
// word
#[derive(Serialize,Deserialize,Debug)]
pub struct Position {
    pub words_to_sentences:HashMap<Ixx,Vec<Ixs>>,
    pub max_length:usize,
    pub min_length:usize
}

impl Position {
    pub fn new() -> Position{
        Position {
            words_to_sentences:HashMap::new(),
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
    pub words_sentences_collections:HashMap<Ixx,Position>,
}

impl TargetWordsToSentences {
    pub fn new() -> TargetWordsToSentences {
        TargetWordsToSentences {
            words_sentences_collections:HashMap::new(),
        }
    }
/*
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
*/
    pub fn from_sentences_dynamics(&mut self
                                   ,sentences:&SentencesAsIndicesDynamicsN
                                   ,no_word:&usize
                                   ,max_target_sentence_len:&usize) {
        let mut hsh = HashMap::new();
        for (ixs,collection) in sentences.fra_words_as_indices {
            for i in &collection.len() {
                hsh.insert()
            }
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
    pub words:HashMap<Ixx,Vec<usize>>,
    pub counts:HashMap<Ixx,HashMap<Ixx,Qxx>>,
}

impl Lengths {
    pub fn new() -> Lengths{
        Lengths {
            words:HashMap::new(),
            counts:HashMap::new(),
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
                let mut map = HashMap::new();
                let mut length;
                for i in collection {
                    length = sentences.eng_words_as_indices
                        .get(i).unwrap().len();
                    coll.push(length);
                    *map.entry(length).or_insert(1)+=1;
                }
                hsh.words.insert(word.to_owned(),coll);
                hsh.counts.insert(word.to_owned(),map);
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

















