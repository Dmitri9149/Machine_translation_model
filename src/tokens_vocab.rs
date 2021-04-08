// vocabulary of tokens at the level of words
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use super::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::cmp::Ordering;
use std::fmt::{self,Debug,Formatter};

/*
// for token indexation
type Ind = usize;
// quantity of tokens in vocabulary (token, quantity)
type Quant = u32;
*/

pub struct VocabOfTokens {
    pub eng_token_quantity:BTreeMap<String,Quant>,
    pub fra_token_quantity:BTreeMap<String,Quant>,
    pub eng_index_quantity:BTreeMap<Ind,Quant>,
    pub fra_index_quantity:BTreeMap<Ind,Quant>,
    pub eng_token_index:BTreeMap<String,Ind>,
    pub fra_token_index:BTreeMap<String,Ind>,
    pub eng_index_token:BTreeMap<Ind,String>,
    pub fra_index_token:BTreeMap<Ind,String>,
    pub eng_token_total:usize,
    pub fra_token_total:usize,

}

impl VocabOfTokens {
    pub fn new() -> VocabOfTokens {
        VocabOfTokens {
    eng_token_quantity:BTreeMap::new(),
    fra_token_quantity:BTreeMap::new(),
    eng_index_quantity:BTreeMap::new(),
    fra_index_quantity:BTreeMap::new(),
    eng_token_index:BTreeMap::new(),
    fra_token_index:BTreeMap::new(),
    eng_index_token:BTreeMap::new(),
    fra_index_token:BTreeMap::new(),
    eng_token_total:0,
    fra_token_total:0,

        }
    }

    pub fn from_word_vocab(&mut self, vocab:&Vocab) {
        let closure = |token_quantity:&mut BTreeMap<String,Quant>
            ,words:&BTreeMap<String,Qxx>| {
            for (word,quant) in words {
                for ch in word.chars() {
                    *token_quantity
                        .entry(ch.to_string())
                        .or_insert(quant.to_owned())+=quant.to_owned();
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
        let closure = |token_quantity:&mut BTreeMap<String,Quant>
            ,token_index:&mut BTreeMap<String,Ind>| {
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


//TODO check the function ? is it used ? 

    pub fn index_to_quantity(&mut self) {
        let closure = |tokens:&BTreeMap<String,Quant>
            ,token_to_index:&BTreeMap<String,Ind>
            ,index_to_quantity:&mut BTreeMap<Ind,Quant>
            | {
                for (token,quantity) in tokens {
                    index_to_quantity.insert(*token_to_index.get(token).unwrap(),*quantity);
                }
            };

        closure(&self.eng_token_quantity,&self.eng_token_index, &mut self.eng_index_quantity);
        closure(&self.fra_token_quantity,&self.fra_token_index, &mut self.fra_index_quantity);

    }


}



