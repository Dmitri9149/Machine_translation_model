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
}

impl FromTo {
    pub fn new() -> Self { FromTo {
        eng_num_words:HashMap::new(),
        fra_num_words:HashMap::new(),
        eng_words_num:HashMap::new(),
        fra_words_num:HashMap::new() }
    }
}
