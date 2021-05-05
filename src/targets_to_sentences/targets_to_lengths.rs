use std::collections::HashMap;
//use std::collections::BTreeMap;
use std::convert::TryInto;
//use std::path::Path;
//use std::fs::read_to_string; // use instead of std::fs::File

use crate::{Ixx,Ixs,Qxx};
use crate::word_dynamics::dynamics::{SentencesAsIndicesDynamicsN};


/*
use std::fmt::{self,Display,Debug,Formatter};
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
*/
/*
use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;
//use ndarray::*;
//use ndarray_linalg::*;
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
use std::fmt::{self,Display,Debug,Formatter};
*/
use std::fmt::{Debug};
use serde::{Serialize,Deserialize};

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
// that sentences iwhich translate with the word as first word
//
#[derive(Serialize, Deserialize,Debug)]
pub struct TargetWordsToSentences {
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
                                   ,no_word:&usize) {
        for len in 0..sentences.target_sentence_max_len {
        let mut hsh = Position::new(); 
        for (ixs,collection) in sentences.fra_words_as_indices.iter() {
            if len <= collection.len() {
                hsh.words_to_sentences
                    .entry(collection[len])
                    .or_insert(vec![*ixs])
                    .push(*ixs);
            } else {
                hsh.words_to_sentences
                    .entry(*no_word)
                    .or_insert(vec![*ixs])
                    .push(*ixs);
            }
        }
        self.words_sentences_collections.insert(len,hsh);
        }
    }


    pub fn max_and_min(&mut self) {
        for (_ixx,position) in self.words_sentences_collections.iter_mut() {
            let mut max = 0;
            let mut min = usize::MAX;
            position.words_to_sentences
                .iter()
                .map(|(_ixs,collection)| {
                    let size = collection.len();
                    if size > max {
                        max=size;
                    }
                    if min > size {
                        min = size;
                    }
                })
                .for_each(drop);
            position.min_length=min;
            position.min_length=min;

        }
    }


}


// for a position in target sentence 
// hash map (words index, vector of sentences lenghts which correspond to the word)
#[derive(Serialize,Deserialize,Debug)]
pub struct TargetLengths {
//    #[serde(serialize_with = "serialize_map_a")]
    pub words_to_lengths:HashMap<Qxx,Vec<usize>>,
    pub lengths_counts:HashMap<Qxx,HashMap<Ixx,Qxx>>,
}

impl TargetLengths {
    pub fn new() -> TargetLengths{
        TargetLengths {
            words_to_lengths:HashMap::new(),
            lengths_counts:HashMap::new(),
        }
    }
}


//map first words in target sentences to the list of sentence lengths (which have the word as 
//first word) , we will have a list like this [2,3,6,3,2,.....] where the numbers are the sentence
//length
//we do the same for second, third....words 
//
#[derive(Serialize, Deserialize,Debug)]
pub struct TargetWordsToSentenceLengths {
    pub words_to_lengths_collections:HashMap<Qxx,TargetLengths>,
}

impl TargetWordsToSentenceLengths {
    pub fn new() -> TargetWordsToSentenceLengths {
        TargetWordsToSentenceLengths {
            words_to_lengths_collections:HashMap::new(),
        }
    }

    pub fn from_words_to_sentences(words:&TargetWordsToSentences
                                   ,sentences:&SentencesAsIndicesDynamicsN) -> TargetWordsToSentenceLengths {
        let mut vectr = HashMap::new();
        for (ind,position) in words.words_sentences_collections.iter() {
        let mut hsh = TargetLengths::new();
            for (word,collection) in &position.words_to_sentences {
                let mut coll = Vec::new();
                let mut map = HashMap::new();
                let mut length;
                for ixs in collection {
                    length = sentences.eng_words_as_indices
                        .get(ixs).unwrap().len();
                    coll.push(length);
                    *map.entry(length).or_insert(1)+=1;
                }
                hsh.words_to_lengths.insert(word.to_owned().try_into().unwrap(),coll);
                hsh.lengths_counts.insert(word.to_owned().try_into().unwrap(),map);
            }
        vectr.insert((*ind).try_into().unwrap(),hsh);

        }

        TargetWordsToSentenceLengths {
            words_to_lengths_collections:vectr,

        }
    }

}

// use to build TargetWordsToSentences structure
// 'no_word' is used to pad the sentences which are shorter 
// than target_max_length 
pub struct Config {
    no_word:Option<usize>,
}

pub struct TargetWordsToSentencesBuilder {
    config: Config,
}

impl Default for TargetWordsToSentencesBuilder {
    fn default() -> Self {
        Self {
            config: Config {
                no_word:None,
            },
        }
    }
}

impl TargetWordsToSentencesBuilder {
    // Constructs a new `TargetWordsToSentencesBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    // Add an 'no_word' 
    pub fn no_word<'a>(&'a mut self, no_word:&usize) -> &'a mut TargetWordsToSentencesBuilder {
        self.config.no_word = Some(*no_word);
        self
    }

    pub fn build(&self,sentences:&SentencesAsIndicesDynamicsN) -> Result<TargetWordsToSentences
        ,Box<dyn std::error::Error>> {
        let mut targets_to_sentences = TargetWordsToSentences::new();
        targets_to_sentences.from_sentences_dynamics(sentences,&self.config.no_word.unwrap());
        targets_to_sentences.max_and_min();

        Ok(targets_to_sentences)
    }
}

















