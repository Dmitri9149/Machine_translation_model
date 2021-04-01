mod translation_corpus;
mod sentence_pairs;
mod word_vocabs;

pub use crate::translation_corpus::{CorpusAsString};
pub use crate::sentence_pairs::{SentencesForTranslation,TranslationPair,
TranslationPairs};
pub use crate::word_vocabs::{Vocab};


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






















