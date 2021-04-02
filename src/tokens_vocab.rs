// vocabulary of tokens at the level of words

use super::*;
use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::fmt::{self,Debug,Formatter};

// for token indexation
type Ind = usize;
// quantity of tokens in vocabulary (token, quantity)
type Quant = u32;

// #[derive (Eq)]
pub struct Token {
    token:String,
    length:usize,
}

pub struct Index {
    index:Ind
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.length, &self.token).cmp(&(other.length, &other.token))
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

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter ) -> fmt::Result {
        write!(f, "Token: {:?} with length: {:?}",
        &self.token,
        &self.length,
    )
    }
}


impl Token {
    pub fn from_data(st:&str) -> Token {
        Token {
        token:st.to_string(),
        length:st.chars().count()
        }
    }

    pub fn new() -> Token {
        Token {
        token:"".to_owned(),
        length:0
        }
    }
}

pub struct VocabOfTokens {
    pub eng_token_quantity:BTreeMap<Token,Quant>,
    pub fra_token_quantity:BTreeMap<Token,Quant>,
    pub eng_token_total:usize,
    pub fra_token_total:usize,

}

impl VocabOfTokens {
    pub fn new() -> VocabOfTokens {
        VocabOfTokens {
    eng_token_quantity:BTreeMap::new(),
    fra_token_quantity:BTreeMap::new(),
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
/*
    pub fn indexation(&mut self) {
        let mut count:usize = 0;
        for (&mut token, _quant) in self.eng_token_quantity.iter_mut() {
            match token.index {
                None => token.index = Some(count),
                Some(x) => panic!("Tokens are already indexed!"),
            }
            count+=1;
        }
    }
*/
}
