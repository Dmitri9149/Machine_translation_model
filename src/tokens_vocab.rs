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
    index:Option<Ind>,
    length:usize,
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
        write!(f, "Token: {:?} with index: {:?} and length: {:?}",
        &self.token,
        &self.index,
        &self.length,
    )
    }
}


impl Token {
    pub fn from_data(st:&str,ind:&Option<Ind>) -> Token {
        Token {
        token:st.to_string(),
        index:*ind,
        length:st.chars().count()
        }
    }

    pub fn new() -> Token {
        Token {
        token:"".to_owned(),
        index:None,
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
        let mut count:Ind = 0;
        let mut token_in_vocab:Option<Token>;
        for (word,quant) in &vocab.eng_words {
            for ch in word.chars() {
                token = Token::from_data(&ch.to_string(),&Some(count));
                token_in_vocab = self.eng_token_quantity.entry(&token);
                match token_in_vocab {
                    Some(x) => 
                        *self.eng_token_quantity
                            .entry(x)
                            .or_insert(0)+=*quant,
                    None => {
                        *self.eng_token_quantity
                            .entry(token)
                            .or_insert(0)+=*quant;
                        count+=1;
                    }   
                }
/*
                *self.eng_token_quantity
                    .entry(token)
                    .or_insert(0)+=*quant;
*/

            }
        }

        let mut token:Token;
        for (word,quant) in &vocab.fra_words {

            for ch in word.chars() {
                token = Token::from_data(&ch.to_string(),&None);
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
        for (&mut token, _quant) in self.eng_token_quantity.iter_mut() {
            match token.index {
                None => token.index = Some(count),
                Some(x) => panic!("Tokens are already indexed!"),
            }
/*
            match self.eng_token_quantity.get(&token) {
                None => 
                    match &token.index {
                        None => token.index = Some(count),
                        Some(x) => panic!("Tokens are already indexed!"),
                    }
                Some(x) => panic!("Can not find the token in vocabulary, smth is wrong!"),
            }
*/
            count+=1;
        }
    }
// to be applied to vocab with (token, quantity) already calculated
    pub fn quantity_of_tokens(&mut self ) {
        self.eng_token_total = self.eng_token_quantity.keys().len();
        self.fra_token_total = self.fra_token_quantity.keys().len();
    }
}
