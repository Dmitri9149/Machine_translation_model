// system in dynamics, merge of most frequent tokens, 
// dynamic changes in vocabulary because of new tokens,
// bookkeeping of new tokens 
use super::*;
use std::collections::HashMap;
use std::collections::BTreeMap;

// keep the records of the flattened tokens as 
// list of indices or as String
pub struct Token {
    pub flattened_to_index:Vec<Ind>,
    pub flattened_to_string:String,
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
    pub pair:(Ind,Ind),
    pub pair_frequency:Quant,
}

pub enum MostFrequentPairLang {
    Eng(MostFrequentPair),
    Fra(MostFrequentPair)
}

impl CandidatesForMerge {
    pub fn from_tokens_words_dynamic(index_token:&BTreeMap<Ind,Token>
                                     ,word_indices:&BTreeMap<Ixx,Vec<Ind>>
                                     ,word_quantity:&BTreeMap<Ixx,Qxx>) -> CandidatesForMerge {
        let mut pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();
        let mut quant:Quant;
//        let mut collection:Vec<Ind> = vec![];
        let mut pair:(Ind,Ind);
        let mut size;
        for (word,collection) in word_indices {
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                quant = *word_quantity.get(&word).unwrap();
                *pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        CandidatesForMerge {
            pairs:pairs
        }

                
    } 


    pub fn from_word_vocab(index_word:&BTreeMap<Ixx,String>
                           ,words_n:&BTreeMap<Ixx,Vec<Ind>>
                           ,numbers:&BTreeMap<Ixx,Qxx>)  -> CandidatesForMerge {
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

    pub fn from_tokens_words_dynamic(dynamics:&TokensAndWordsDynamicsLang) 
        -> CandidatesForMergeLang {
            match dynamics {
                TokensAndWordsDynamicsLang::Eng(x) => 
                    CandidatesForMergeLang
                    ::Eng(CandidatesForMerge
                          ::from_tokens_words_dynamic(&x.index_token
                                                      ,&x.word_indices
                                                      ,&x.word_quantity)),

                TokensAndWordsDynamicsLang::Fra(y) => 
                    CandidatesForMergeLang
                    ::Fra(CandidatesForMerge
                          ::from_tokens_words_dynamic(&y.index_token
                                                      ,&y.word_indices
                                                      ,&y.word_quantity)),
            }
    }


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

pub struct TokensAndWordsDynamics {
// TODO is it possible to use &str instead of String ? with reference to token.flattened_to_string?
    pub index_token:BTreeMap<Ind,Token>,
    pub token_index:BTreeMap<String,Ind>,
    pub word_quantity:BTreeMap<Ixx,Qxx>,
    pub word_indices:BTreeMap<Ixx,Vec<Ind>>
    
}


impl TokensAndWordsDynamics {
    pub fn new() -> TokensAndWordsDynamics {
        TokensAndWordsDynamics {
            index_token:BTreeMap::new()
                ,token_index:BTreeMap::new()
                ,word_quantity:BTreeMap::new()
                ,word_indices:BTreeMap::new()
        }
    }

    pub fn initial_set_from_vocab(index_word:&BTreeMap<Ixx,String>
                                  ,index_token:&BTreeMap<Ind,String>
                                  ,token_index:&BTreeMap<String,Ind>
                                  ,word_quantity:&BTreeMap<Ixx,Qxx>) -> TokensAndWordsDynamics {
        let mut hsh_index:BTreeMap<Ind,Token> = BTreeMap::new();
        let mut hsh_token:BTreeMap<String,Ind> = BTreeMap::new();
// TODO rewrite to:  for (index,token) in index_token { .... }
        for index in index_token {
            let st = index.1.to_string();
            let token = Token {
                flattened_to_index:vec![*index.0],
                flattened_to_string:st.to_owned()
            };
// TODO check for containing of the index key -> generate corresp behaviour
            hsh_index.entry(*index.0).or_insert(token);
            hsh_token.entry(st).or_insert(*index.0);
        }

        let mut hsh_word:BTreeMap<Ixx,Vec<Token>> = BTreeMap::new();
        let mut hsh_word_ics:BTreeMap<Ixx,Vec<Ind>> = BTreeMap::new();
        let mut char_index:Ind;
        let mut char_as_string:String;
        for (index,word) in index_word {
            let mut vec_of_indices:Vec<Ind>=Vec::new();
            for ch in word.chars() {
                char_as_string = ch.to_string();
                char_index = *token_index.get(&char_as_string).unwrap();
                vec_of_indices.push(char_index);
            }
            hsh_word_ics.entry(*index).or_insert(vec_of_indices);
        }

        TokensAndWordsDynamics {
            index_token:hsh_index
            ,token_index:hsh_token
            ,word_indices:hsh_word_ics
            ,word_quantity:word_quantity.to_owned()
        }
    }
 
    pub fn from_most_frequent_pair(&mut self,pair:&MostFrequentPair) {
        let mut to_index_left = self.index_token.get(&pair.pair.0).unwrap().flattened_to_index.to_vec();
        let mut to_index_right = self.index_token.get(&pair.pair.1).unwrap().flattened_to_index.to_vec();
        to_index_left.append(&mut to_index_right);

        let mut to_string_left = self.index_token.get(&pair.pair.0).unwrap().flattened_to_string.to_owned();
        let to_string_right = self.index_token.get(&pair.pair.1).unwrap().flattened_to_string.to_owned();
        to_string_left.push_str(&to_string_right);
        let st = &to_string_left.to_owned();

        let token = Token {flattened_to_index:to_index_left,flattened_to_string:to_string_left};
        
        let size = self.index_token.keys().len();
        let new_index = size +1;
        if self.index_token.contains_key(&new_index) {
            panic!("The new key already exist: {:?} ; panic!", new_index);
        }
        self.index_token.insert(new_index,token);
// TODO what to do if "to_string_left" already exist ?
        if self.token_index.contains_key(st) {
            panic!("The string key already exist: {:?}",st.to_string());
        }
        self.token_index.entry(st.to_string()).or_insert(new_index);
// TODO find needed pair in vector of numbers and change the pair to a new number
//
//        find_and_change_in_place_pair(&mut self,&pair,new:&Ind) 
        
        self.word_indices
        .iter_mut()
        .map(|(index,mut vector)| find_and_change_in_place_pair(vector,&pair.pair,&new_index))
        .collect()    
    }
}

pub enum TokensAndWordsDynamicsLang {
    Eng(TokensAndWordsDynamics),
    Fra(TokensAndWordsDynamics)
}

impl TokensAndWordsDynamicsLang {
    pub fn new(lang:Lang) -> TokensAndWordsDynamicsLang {
        match lang {
            Lang::Eng => TokensAndWordsDynamicsLang::Eng(TokensAndWordsDynamics::new()),
            Lang::Fra => TokensAndWordsDynamicsLang::Fra(TokensAndWordsDynamics::new()),
        }
    }

    pub fn initial_set_from_vocab(lang:Lang
                                  ,vocab_t:&VocabOfTokens
                                  ,vocab_w:&Vocab) -> TokensAndWordsDynamicsLang{
        match lang {
            Lang::Eng  => TokensAndWordsDynamicsLang
                ::Eng(TokensAndWordsDynamics::initial_set_from_vocab(&vocab_w.eng_index_word
                                                            ,&vocab_t.eng_index_token
                                                            ,&vocab_t.eng_token_index
                                                            ,&vocab_w.eng_numbers)),
            Lang::Fra  => TokensAndWordsDynamicsLang
                ::Fra(TokensAndWordsDynamics::initial_set_from_vocab(&vocab_w.fra_index_word
                                                            ,&vocab_t.fra_index_token
                                                            ,&vocab_t.fra_token_index
                                                            ,&vocab_w.fra_numbers)),
        }
    }

    pub fn from_most_frequent_pair(&mut self,pair:&MostFrequentPairLang) {
        match self {
            TokensAndWordsDynamicsLang::Eng(x) => 
                match pair {
                    MostFrequentPairLang::Eng(y) => x.from_most_frequent_pair(y),
                    _ => panic!("TokensDynamicLang error: Source...Target language data inconsistency"),
                }
            TokensAndWordsDynamicsLang::Fra(x) => 
                match pair {
                    MostFrequentPairLang::Fra(y) => x.from_most_frequent_pair(y),
                    _ => panic!("TokensDynamicLang error: Source...Target language data inconsistency"),
                }
        }
    }
}


