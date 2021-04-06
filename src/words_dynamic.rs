//
//use super::*;
use std::collections::HashMap;

// dynamically changing Vocab of Words which are represented 
// as map from numbers (word indices) to collection of token indices
// the dynamic is because new tokens (and new indixes) are generated 
// while the tokenizer is runnind

pub struct WordAsTokensDynamic {

    word_as_indices:HashMap<Ixx,Vec<Ind>>,
    word_as_collection:HashMap<Ixx,Vec<Vec<Ind>>>
}

pub enum WordAsTokensDynamicLang {
    Eng(WordAsTokensDynamic),
    Fra(WordAsTokensDynamic)
}

impl WordAsTokensDynamic {
    pub fn at_the_beginning(words_n:&HashMap<Ixx,Vec<Ind>>) -> WordAsTokensDynamic{
        let mut hsh:HashMap<Ixx,Vec<Vec<Ind>>> = HashMap::new();
        for word in words_n {
// TODO what if there is already an entry
            hsh.entry(*word.0).or_insert(vec![word.1.to_vec()]);
        }

        WordAsTokensDynamic {
            word_as_indices:words_n.to_owned(),
            word_as_collection:hsh
        }
    }

    pub fn words_dynamic (&mut self, )
}

impl WordAsTokensDynamicLang {
    pub fn at_the_beginning(lang:Lang,collections:&WordToIndexCollection) 
        -> WordAsTokensDynamicLang {
            match lang {
                Lang::Eng => WordAsTokensDynamicLang
                    ::Eng(WordAsTokensDynamic::at_the_beginning(&collections.eng_words_n)),
                Lang::Fra => WordAsTokensDynamicLang
                    ::Fra(WordAsTokensDynamic::at_the_beginning(&collections.fra_words_n)),
            }
    }
}
