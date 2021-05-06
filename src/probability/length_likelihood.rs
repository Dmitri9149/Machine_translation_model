use std::collections::HashMap;
//use std::collections::BTreeMap;
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
    pub lengths_counts:HashMap<Qxx,HashMap<u16,u32>>,
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
                    coll.push(length.try_into().unwrap());
                    *map.entry(length.try_into().unwrap()).or_insert(1)+=1;
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

#[derive(Serialize,Deserialize,Debug)]
pub struct TargetWordsCount {
    pub words_counts:HashMap<Ixx,Qxx>,
}

impl TargetWordsCount {
    pub fn new() -> TargetWordsCount{
        TargetWordsCount {
            words_counts:HashMap::new(),
        }
    }
}

pub struct PositionalTargetWordsCount {
    targets_words_and_lengths:HashMap<Ixx,TargetWordsCount>,
}

impl PositionalTargetWordsCount {
    pub fn new() -> PositionalTargetWordsCount {
        PositionalTargetWordsCount {
            targets_words_and_lengths:HashMap::new()
        }
    }

    pub fn from_target_words_to_sentence_lengths(&mut self
                                                 ,lengths:&TargetWordsToSentenceLengths) {
        for (position, collection) in lengths.words_to_lengths_collections.iter() {
            let mut map = TargetWordsCount::new();
            for (ixx,vec) in collection.words_to_lengths.iter() {
                map.words_counts
                    .insert((*ixx).try_into().unwrap(),(vec.len()).try_into().unwrap());
            }
            self.targets_words_and_lengths
                .insert((*position).try_into().unwrap(),map);
        }
    }
}














