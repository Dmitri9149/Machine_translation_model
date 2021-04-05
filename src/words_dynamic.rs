// description of token 
//
use super::*;
use std::collections::HashMap;

// dynamically changing Vocab of Words which are represented 
// as map from numbers (word indices) to collection of token indices
// the dynamic is because new tokens (and new indixes) are generated 
// while the tokenizer is runnind

pub struct WordAsTokensDynamic {
    words_as_indices:HashMap<Ixx,Vec<Ind>>,
    words_as_tokens:HashMap<Ixx,String>
}

pub enum WordAsTokensDynamicLang {
    Eng(WordAsTokensDynamic),
    Fra(WordAsTokensDynamic)
}

impl WordAsTokensDynamic {
    pub fn at_the_beginning(words_n:&HashMap<Ixx,Vec<Ind>>
                            ,index_word:&HashMap<Ixx,String>) -> WordAsTokensDynamic{
        WordAsTokensDynamic {
            words_as_indices:words_n.to_owned(),
            words_as_tokens:index_word.to_owned()
        }
    }
}

impl WordAsTokensDynamicLang {
    pub fn at_the_beginning(lang:Lang,vocab:&Vocab,collections:&WordToIndexCollection) 
        -> WordAsTokensDynamicLang {
            match lang {
                Lang::Eng => WordAsTokensDynamicLang
                    ::Eng(WordAsTokensDynamic::at_the_beginning(&collections.eng_words_n,&vocab.eng_index_word)),
                Lang::Fra => WordAsTokensDynamicLang
                    ::Fra(WordAsTokensDynamic::at_the_beginning(&collections.fra_words_n,&vocab.fra_index_word)),
            }
    }
}
