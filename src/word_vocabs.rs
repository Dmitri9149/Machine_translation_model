// vocabulary for eng and fra corpus , on the level of words
use std::collections::HashMap;
use super::*;

// is used for calculating quantity of words
type Qxx = u32;
// is used for the indexing (numbering) of words
type Ixx = u16;

pub struct Vocab {
// the list of all words (may repeat) which we can get by splitting all 
// sentences (usually we split on white space)
    pub eng_set:Vec<String>,
    pub fra_set:Vec<String>,
// (word, quantity of the words) in our corpus
    pub eng_words:HashMap<String,Qxx>,
    pub fra_words:HashMap<String,Qxx>,
// (number, quantity of the number) number here in the number equivalent of a word
    pub eng_numbers:HashMap<Ixx,Qxx>,
    pub fra_numbers:HashMap<Ixx,Qxx>,
// from number equivalent to word representation
    pub eng_num_words:HashMap<Ixx,String>,
    pub fra_num_words:HashMap<Ixx,String>,
// from word to its number representation
    pub eng_words_num:HashMap<String,Ixx>,
    pub fra_words_num:HashMap<String,Ixx>,
// number of words in fra or eng 
    pub eng_words_total:Ixx,
    pub fra_words_total:Ixx,

}


impl Vocab {
    pub fn new() -> Self { Vocab {
        eng_set:vec![],
        fra_set:vec![],
        eng_words:HashMap::new(),
        fra_words:HashMap::new(),
        eng_numbers:HashMap::new(),
        fra_numbers:HashMap::new(),
        eng_num_words:HashMap::new(),
        fra_num_words:HashMap::new(),
        eng_words_num:HashMap::new(),
        fra_words_num:HashMap::new(),
        eng_words_total:0,
        fra_words_total:0,
    }
    }

    pub fn list_of_words(&mut self, vector_sentences:&SentencesForTranslation) {
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

    pub fn words_and_quantity(&mut self) {
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

        self.eng_words_total = self.eng_words.keys().len() as Ixx;
        self.fra_words_total = self.fra_words.keys().len() as Ixx;

    }
// unique numbers are attached to every word in vocab and this is stored in hash structures
// so we can go from word to its number representation and back
    pub fn indexation_from_hash(&mut self) {
        for word in &self.eng_words {
            let count = self.eng_words_num
                .entry(word.0.to_owned())
                .or_insert(0);
            *count+=1;

        }

        for word in &self.fra_words {
            let count = self.fra_words_num
                .entry(word.0.to_owned())
                .or_insert(0);
            *count+=1;
        }
    }
}

