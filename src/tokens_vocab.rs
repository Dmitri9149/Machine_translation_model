// vocabulary of tokens at the level of words
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use super::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;
use std::fmt::{self,Debug,Formatter};

// for token indexation
type Ind = usize;
// quantity of tokens in vocabulary (token, quantity)
type Quant = u32;


pub struct VocabOfTokens {
    pub eng_token_quantity:HashMap<String,Quant>,
    pub fra_token_quantity:HashMap<String,Quant>,
    pub eng_token_index:HashMap<String,Ind>,
    pub fra_token_index:HashMap<String,Ind>,
    pub eng_index_token:HashMap<Ind,String>,
    pub fra_index_token:HashMap<Ind,String>,
    pub eng_token_total:usize,
    pub fra_token_total:usize,

}

impl VocabOfTokens {
    pub fn new() -> VocabOfTokens {
        VocabOfTokens {
    eng_token_quantity:HashMap::new(),
    fra_token_quantity:HashMap::new(),
    eng_token_index:HashMap::new(),
    fra_token_index:HashMap::new(),
    eng_index_token:HashMap::new(),
    fra_index_token:HashMap::new(),
    eng_token_total:0,
    fra_token_total:0,

        }
    }

    pub fn from_word_vocab(&mut self, vocab:&Vocab) {
        let closure = |token_quantity:&mut HashMap<String,Quant>
            ,words:&HashMap<String,Qxx>| {
            for (word,quant) in words {
                for ch in word.chars() {
                    *token_quantity
                        .entry(ch.to_string())
                        .or_insert(0)+=quant.to_owned();
                }
            }
        };
        closure(&mut self.eng_token_quantity,&vocab.eng_words);
        closure(&mut self.fra_token_quantity,&vocab.fra_words);
        self.eng_token_total = self.eng_token_quantity.keys().len();
        self.fra_token_total = self.fra_token_quantity.keys().len();
    }

// same as token_to_index but  closure is used; map token to index
    pub fn token_to_index_c(&mut self) {
        let closure = |token_quantity:&mut HashMap<String,Quant>
            ,token_index:&mut HashMap<String,Ind>| {
            let mut count:Ind = 0;
            for (token, _) in token_quantity {
                token_index.insert(token.to_string(),count);
                count+=1;
            }
        };
        closure(&mut self.eng_token_quantity,&mut self.eng_token_index);
        closure(&mut self.fra_token_quantity,&mut self.fra_token_index);
    }
   
// map token to index
    pub fn token_to_index(&mut self) {
        let mut count:Ind = 0;
        for (token, _) in &self.eng_token_quantity {
            self.eng_token_index.insert(token.to_string(),count);
            count+=1;
        }

        let mut count:Ind = 0;
        for (token, _) in &self.fra_token_quantity {
            self.fra_token_index.insert(token.to_string(),count);
            count+=1;
        }

    }

// map index to token
    pub fn index_to_token(&mut self) {
        for (token, ind) in &self.eng_token_index {
            self.eng_index_token.insert(*ind,token.to_string());
        }

        for (token, ind) in &self.fra_token_index {
            self.fra_index_token.insert(*ind,token.to_string());
        }

    }

}