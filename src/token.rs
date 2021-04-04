// description of token 
//
use super::*;
use std::collections::HashMap;

pub struct Token {
//    eng_token:HashMap<Ind,Vec<Ind>>,
//    fra_token:HashMap<Ind,Vec<Ind>>,
    eng_flattened_to_index:Vec<Ind>,
    fra_flattened_to_index:Vec<Ind>,
    eng_flattened_to_chars:Vec<String>,
    fra_flattened_to_chars:Vec<String>,
}
// dynamically changing Vocab of Words which are represented 
// as map from numbers (word indices) to collection of 
// the dynamic is because new tokens (and new indixes) are generated 
// while the tokenizer is runnind
pub struct WordAsTokensDynamic {
    eng_words:HashMap<Ixx,Vec<Ind>>,
    fra_words:HashMap<Ixx,Vec<Ind>>,


}

impl WordAsTokensDynamic {
    pub fn at_the_beginning(words:WordToIndexCollection) -> WordAsTokensDynamic{
        WordAsTokensDynamic {
            eng_words:words.eng_words_n,
            fra_words:words.fra_words_n,
        }
    }
}


