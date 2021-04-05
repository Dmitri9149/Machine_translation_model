// description of token 
//
use super::*;
use std::collections::HashMap;

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


