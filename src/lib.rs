use std::collections::HashMap;

mod translation_corpus;
mod sentence_pairs;
mod word_vocabs;
mod tokens_vocab;
mod system_dynamic;
mod words_dynamic;

pub use crate::translation_corpus::{CorpusAsString};
pub use crate::sentence_pairs::{SentencesForTranslation,TranslationPair,
TranslationPairs};
pub use crate::word_vocabs::{Vocab,WordToIndexCollection};
pub use tokens_vocab::{VocabOfTokens};
pub use system_dynamic::{CandidatesForMerge,MostFrequentPair,TokensDynamic,Token};
pub use words_dynamic::{WordAsTokensDynamic,WordAsTokensDynamicLang};

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
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<(&K,&V)>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
//        .map(|(k, v)| (k,v))
}



















