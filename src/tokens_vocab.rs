// vocabulary of tokens at the level of words

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

// #[derive (Eq)]
pub struct Token {
    token:String,
}

#[derive (Copy,Clone)]
pub struct Index {
    index:Ind
}


impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        self.token.cmp(&other.token)
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.token.hash(hasher);
    }
}
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter ) -> fmt::Result {
        write!(f, "Token: {:?}",
        &self.token,
    )
    }
}


impl Token {
    pub fn from_data(st:&str) -> Token {
        Token {
        token:st.to_string(),
        }
    }

    pub fn new() -> Token {
        Token {
        token:"".to_owned(),
        }
    }
}

impl Index {
    pub fn from_number(count:&Ind) -> Index {
        Index {
            index:*count
        
        }
    }
}

pub struct VocabOfTokens {
    pub eng_token_quantity:HashMap<Token,Quant>,
    pub fra_token_quantity:HashMap<Token,Quant>,
    pub eng_token_index:HashMap<Token,Index>,
    pub fra_token_index:HashMap<Token,Index>,
    pub eng_index_token:HashMap<Index,Token>,
    pub fra_index_token:HashMap<Index,Token>,
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
        let mut token:Token;
        for (word,quant) in &vocab.eng_words {

            for ch in word.chars() {
                token = Token::from_data(&ch.to_string());
                *self.eng_token_quantity
                    .entry(token)
                    .or_insert(0)+=quant.to_owned();

            }
        }



        let mut token:Token;
        for (word,quant) in &vocab.fra_words {

            for ch in word.chars() {
                token = Token::from_data(&ch.to_string());
                *self.fra_token_quantity
                    .entry(token)
                    .or_insert(0)+=quant.to_owned();

            }
        }


        self.eng_token_total = self.eng_token_quantity.keys().len();
        self.fra_token_total = self.fra_token_quantity.keys().len();


    }

    pub fn indexation(&mut self) {
        let mut count:usize = 0;
        let mut index:Index;
        let mut insert:Token;
        for (token, _) in &self.eng_token_quantity {
            index = Index::from_number(&count);
            insert = Token::from_data(&token.token);
            self.eng_token_index.insert(insert,index);
            count+=1;
        }
    }

}
