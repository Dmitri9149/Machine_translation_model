// vocabulary for eng and fra corpus , on the level of words
use std::collections::HashMap;
use super::*;

pub struct Vocab {
    pub eng_set:Vec<String>,
    pub fra_set:Vec<String>,
    pub eng_words:HashMap<String,u32>,
    pub fra_words:HashMap<String,u32>,
    pub eng_numbers:HashMap<u32,u32>,
    pub fra_numbers:HashMap<u32,u32>
}

// from words to numbers and back
pub struct FromTo {
    pub eng_num_words:HashMap<u32,String>,
    pub fra_num_words:HashMap<u32,String>,
    pub eng_words_num:HashMap<String,u32>,
    pub fra_words_num:HashMap<String,u32>
}

impl Vocab {
    pub fn new() -> Self { Vocab {
        eng_set:vec![],
        fra_set:vec![],
        eng_words:HashMap::new(),
        fra_words:HashMap::new(),
        eng_numbers:HashMap::new(),
        fra_numbers:HashMap::new(), }
    }

    pub fn vector_words(&mut self, vector_sentences:&SentencesForTranslation) {
        let size = vector_sentences.eng.len();
        let mut res_eng:Vec<String>=Vec::with_capacity(size);
        let mut res_fra:Vec<String>=Vec::with_capacity(size);
        for sentence in &vector_sentences.eng {
            for word in sentence.trim().split_whitespace(){
                res_eng.push(word.to_owned());
            }
        }

        for sentence in &vector_sentences.fra {
            for word in sentence.trim().split_whitespace(){
                res_fra.push(word.to_owned());
            }
        }

        self.eng_set=res_eng;
        self.fra_set=res_fra;   
    }

    pub fn words_to_vocab(& mut self) {
        for word in self.eng_set.iter() {
            let count = self
                .eng_words
                .entry(word.to_owned())
                .or_insert(0);
            *count+=1;
        }
        for word in self.fra_set.iter() {
            let count = self
                .fra_words
                .entry(word.to_owned())
                .or_insert(0);
            *count+=1;
        }

    }
}

impl FromTo {
    pub fn new() -> Self { FromTo {
        eng_num_words:HashMap::new(),
        fra_num_words:HashMap::new(),
        eng_words_num:HashMap::new(),
        fra_words_num:HashMap::new() }
    }
}
