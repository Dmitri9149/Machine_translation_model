use std::collections::HashMap;
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt::{Debug};
use serde::{Serialize,Deserialize};

//use std::path::Path;
//use std::fs::read_to_string; // use instead of std::fs::File

use crate::{Ixx,Ixs,Qxx};
use crate::word_dynamics::dynamics::{SentencesAsIndicesDynamicsN};
use crate::targets_to_sentences::targets_to_lengths::{
    TargetWordsToSentences
};

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
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
use std::fmt::{self,Display,Debug,Formatter};
*/

// for a position in target sentence 
// hash map (words index, vector of sentences lenghts which correspond to the word)
#[derive(Serialize,Deserialize,Debug)]
pub struct TargetLengths {
//    #[serde(serialize_with = "serialize_map_a")]
    pub words_to_lengths:HashMap<Qxx,Vec<u16>>,
    pub lengths_counts:HashMap<Qxx,BTreeMap<u16,u32>>,
    pub words_counts:HashMap<Qxx,Qxx>,
    pub lengths_likelihood:HashMap<Qxx,BTreeMap<u16,f64>>,
}

impl TargetLengths {
    pub fn new() -> TargetLengths{
        TargetLengths {
            words_to_lengths:HashMap::new(),
            lengths_counts:HashMap::new(),
            words_counts:HashMap::new(),
            lengths_likelihood:HashMap::new(),
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
    pub words_to_lengths_collections:BTreeMap<u16,TargetLengths>,
}

impl TargetWordsToSentenceLengths {
    pub fn new() -> TargetWordsToSentenceLengths {
        TargetWordsToSentenceLengths {
            words_to_lengths_collections:BTreeMap::new(),
        }
    }

    pub fn from_words_to_sentences(words:&TargetWordsToSentences
                                   ,sentences:&SentencesAsIndicesDynamicsN) -> TargetWordsToSentenceLengths {
        let mut vectr = BTreeMap::new();
        for (ind,position) in words.words_sentences_collections.iter() {
        let mut hsh = TargetLengths::new();
            for (word,collection) in &position.words_to_sentences {
                let mut coll = Vec::new();
                let mut map = BTreeMap::new();
                let mut length;
                if collection.len() == 0 {
                    panic!("A target word has 0 lengths collection of entences!");
                }
                for ixs in collection {
                    length = sentences.eng_words_as_indices
                        .get(ixs).unwrap().len();
                    coll.push(length.try_into().unwrap());
                    *map.entry(length.try_into().unwrap()).or_insert(0)+=1;
                }
                hsh.words_to_lengths.insert(word.to_owned().try_into().unwrap(),coll);
                hsh.lengths_counts.insert(word.to_owned().try_into().unwrap(),map);
                hsh.words_counts.insert(word.to_owned().try_into().unwrap()
                                        ,collection.len().try_into().unwrap());

            }
        vectr.insert((*ind).try_into().unwrap(),hsh);

        }
        TargetWordsToSentenceLengths {
            words_to_lengths_collections:vectr,
        }
    }

    pub fn lengths_likelihood(&mut self, sentences_max_len:&u16) {
        for (position,  targ_lengths) in self.words_to_lengths_collections.iter_mut() {
            for (qxx,collection) in targ_lengths.lengths_counts.iter() {
                let qxx_total = targ_lengths.words_counts
                    .get(qxx)
                    .unwrap();
                let mut likely:BTreeMap<u16,f64>=BTreeMap::new();
// Laplace smoothening is below
                for (len,count) in collection.iter() {
                    *likely
                        .entry(*len)
                        .or_insert(0.0)
                        +=(f64::from(*count)+1.0)/(f64::from(*qxx_total) + f64::from(*sentences_max_len));
                }
                for i in 0..*sentences_max_len {
                    likely
                        .entry(i+1)
                        .or_insert(1.0/(f64::from(*qxx_total) + f64::from(*sentences_max_len)));
                }
                targ_lengths.lengths_likelihood
                    .insert(*qxx,likely);
            }
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TargetWordsCount {
    pub words_counts:HashMap<Ixx,Qxx>,
    pub words_total:Qxx
}

impl TargetWordsCount {
    pub fn new() -> TargetWordsCount{
        TargetWordsCount {
            words_counts:HashMap::new(),
            words_total:0,
        }
    }
}

pub struct PositionalTargetWordsCount {
    target_words_and_lengths:BTreeMap<u16,TargetWordsCount>,
}

impl PositionalTargetWordsCount {
    pub fn new() -> PositionalTargetWordsCount {
        PositionalTargetWordsCount {
            target_words_and_lengths:BTreeMap::new()
        }
    }

    pub fn from_target_words_to_sentence_lengths(&mut self
                                                 ,lengths:&TargetWordsToSentenceLengths) {
        for (position, collection) in lengths.words_to_lengths_collections.iter() {
            let mut map = TargetWordsCount::new();
            for (ixx,vec) in collection.words_to_lengths.iter() {
                map.words_counts
                    .insert((*ixx).try_into().unwrap(),(vec.len()).try_into().unwrap());
                map.words_total = map.words_total + vec.len() as u32;
            }
            self.target_words_and_lengths
                .insert((*position).try_into().unwrap(),map);
        }
    }
}

// same as above but integer counts are converted to probabilities
pub struct TargetWordsProbability {
    words_probability:HashMap<Ixx,f64>,
    words_total:Qxx,
}

impl TargetWordsProbability {
    pub fn new() -> TargetWordsProbability {
        TargetWordsProbability  {
        words_probability:HashMap::new(),
        words_total:0,
        }
    }
}

pub struct PositionalTargetWordsProbability {
    positional_words_probability:BTreeMap<u16,TargetWordsProbability>,
}

impl PositionalTargetWordsProbability {
    pub fn new(counts:&PositionalTargetWordsCount) -> PositionalTargetWordsProbability {
        let mut maps = BTreeMap::new();

        for (position,collection) in counts.target_words_and_lengths.iter() {
            let mut hsh = TargetWordsProbability::new();
            let total = counts.target_words_and_lengths
                .get(position)
                .unwrap()
                .words_total;
                if total == 0 {
                    panic!("Words total at position:{} is 0",&position);
                }
            for (ixx, quantity) in collection.words_counts.iter() {
                hsh.words_probability.insert(*ixx,f64::from(*quantity)/f64::from(total));
                hsh.words_total=total;
            }
        maps.insert((*position).try_into().unwrap(),hsh);
        }
        PositionalTargetWordsProbability {
            positional_words_probability:maps,
        }   
    }
}














