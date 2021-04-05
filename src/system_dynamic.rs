// system in dynamics, merge of most frequent tokens, 
// dynamic changes in vocabulary because of new tokens,
// bookkeeping of new tokens 
use super::*;
use std::collections::HashMap;

// keep the records of the flattened tokens as 
// list of indices or as String
pub struct Token {
    flattened_to_index:Vec<Ind>,
    flattened_to_string:String,
}

pub enum Lang {
    Eng,
    Fra
}
pub enum TokenLang {
    Eng(Token),
    Fra(Token)
}

pub struct CandidatesForMerge {
    pub pairs:HashMap<(Ind,Ind),Quant>,
}

pub enum CandidatesForMergeLang {
    Eng(CandidatesForMerge),
    Fra(CandidatesForMerge)
}

pub struct MostFrequentPair {
    pair:(Ind,Ind),
    pair_frequency:Quant,
}

pub enum MostFrequentPairLang {
    Eng(MostFrequentPair),
    Fra(MostFrequentPair)
}

impl CandidatesForMerge {

    pub fn from_word_vocab(index_word:&HashMap<Ixx,String>
                           ,words_n:&HashMap<Ixx,Vec<Ind>>
                           ,numbers:&HashMap<Ixx,Qxx>)  -> CandidatesForMerge {
        let mut pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();
        let mut quant:Quant;
        let mut collection:Vec<Ind> = vec![];
        let mut pair:(Ind,Ind);
        let mut size;
        for word in index_word.keys() {
            collection = words_n.get(&word).unwrap().to_vec();
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                quant = *numbers.get(&word).unwrap();
                *pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        CandidatesForMerge {
            pairs:pairs
        }
    }

    pub fn most_frequent_pair(&self) -> MostFrequentPair {
        let closure = |pairs:&HashMap<(Ind,Ind),Quant>| {
            let res = max_key(pairs).expect("The vocabulary is to be not empty");
            (*res.0,*res.1)
        };
        let max_pair = closure(&self.pairs);
        MostFrequentPair {
        pair:max_pair.0,    
        pair_frequency:max_pair.1,
        }
    }

}

impl CandidatesForMergeLang {
    pub fn from_word_vocab(vocab:&Vocab, collection:&WordToIndexCollection,lang:Lang) 
        -> CandidatesForMergeLang {
            match lang {
                Lang::Eng => 
                    CandidatesForMergeLang::Eng(CandidatesForMerge::from_word_vocab(
                            &vocab.eng_index_word
                            ,&collection.eng_words_n
                            ,&vocab.eng_numbers
                                                           )),

                Lang::Fra => 
                    CandidatesForMergeLang::Eng(CandidatesForMerge::from_word_vocab(
                            &vocab.fra_index_word
                            ,&collection.fra_words_n
                            ,&vocab.fra_numbers
                                                           )),

            }
        }
}


impl MostFrequentPairLang {
    pub fn most_frequent_pair(candidates:&CandidatesForMergeLang) -> MostFrequentPairLang {
        match candidates {
            CandidatesForMergeLang::Eng(x) 
                => MostFrequentPairLang::Eng(CandidatesForMerge::most_frequent_pair(&x)),
            CandidatesForMergeLang::Fra(y) 
                => MostFrequentPairLang::Fra(CandidatesForMerge::most_frequent_pair(&y))
        }
    }
}


pub struct NewToken {
    new_token:Token,
}

pub enum NewTokenLang {
    Eng(NewToken),
    Fra(NewToken)
}


/*
// keep records of all new + initial ('letters') tokens and 
// the indices of the tokens
*/
pub struct TokensDynamic {
    index_token:HashMap<Ind,Token>,
}

impl TokensDynamic {
    pub fn new() -> TokensDynamic {
        TokensDynamic {index_token:HashMap::new()}
    }

    pub fn initial_set_from_vocab(index_token:&HashMap<Ind,String>) -> TokensDynamic {
        let mut hsh:HashMap<Ind,Token> = HashMap::new();
        for index in index_token {
            let token = Token {
                flattened_to_index:vec![*index.0],
                flattened_to_string:index.1.to_string()
            };
            hsh.entry(*index.0).or_insert(token);
        }

        TokensDynamic {index_token:hsh}
    }
}

pub enum TokensDynamicLang {
    Eng(TokensDynamic),
    Fra(TokensDynamic)
}

impl TokensDynamicLang {
    pub fn new(lang:Lang) -> TokensDynamicLang {
        match lang {
            Lang::Eng => TokensDynamicLang::Eng(TokensDynamic::new()),
            Lang::Fra => TokensDynamicLang::Fra(TokensDynamic::new()),
        }
    }

    pub fn initial_set_from_vocab(lang:Lang,vocab:&VocabOfTokens) -> TokensDynamicLang{
        match lang {
            Lang::Eng  => TokensDynamicLang::Eng(TokensDynamic::initial_set_from_vocab(&vocab.eng_index_token)),
            Lang::Fra  => TokensDynamicLang::Fra(TokensDynamic::initial_set_from_vocab(&vocab.fra_index_token)),
        }
    }

/*
    pub fn initial_set(&mut self,vocab:&VocabOfTokens) {
            for index_eng,index_fra in (vocab.eng_index_token, vocab.fra_index_token) {
                let token = 
                    Token {
                        eng_flattened_to_index_eng:vec![index.0],
                        eng_flattened_to_string:index_eng.1,
                    }
                self.eng_index_token.entry(index.0).or_insert(token);
            }
    }
*/
}



