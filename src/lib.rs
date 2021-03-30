mod translation_corpus;

pub use crate::translation_corpus::{CorpusAsString};

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


