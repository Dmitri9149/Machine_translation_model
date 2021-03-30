// read text file which contains translation pairs and save it 
// as one big string, some preprocessing at the level of the 
// whole string is possible (like add spaces between ? mark and end of sentences)
//
use std::fs::File;
use std::io::prelude::*;

use super::*;

pub struct CorpusAsString {
    pub original:String,
    pub processed:String

}


impl CorpusAsString {
// build by reading a file
    pub fn corpus_from_file(path: &str) -> CorpusAsString {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        CorpusAsString {
            original: contents.clone(),
            processed:contents
        }
    }

    pub fn replace_some_chars(&self, aa:&str, b:char) -> Self {
        let res = replace_chars_to_char(&self.original,aa,b);
        CorpusAsString {
            processed:res, original:self.original.to_owned()
        }
    }
}
 
