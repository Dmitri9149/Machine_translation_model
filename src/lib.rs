mod translation_corpus;

pub use crate::translation_corpus::{CorpusAsString};

pub fn replace_char_to_char(stng:&str, aa:&str, b:char) -> String {
    stng
        .chars()
        .map(|x| -> char {
            if aa.contains(x) {
                b
            } else {x}
        })
        .collect()
}
