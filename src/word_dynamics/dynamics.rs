//eng_fra structures are collected her in Struct (product structures) not id Enum structures

// system in dynamics, merge of most frequent tokens, 
// dynamic changes in vocabulary because of new tokens,
// bookkeeping of new tokens 
//use translationlib::*;
use super::super::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::{self,Debug,Formatter};
use serde::{Serialize, Deserialize};


// keep the records of the flattened tokens as 
// list of indices or as String
pub struct TokenN {
    pub flattened_to_index:Vec<Ind>,
    pub flattened_to_string:String,
}

#[derive(Debug)]
pub struct WordsAsTokensN {
    pub eng_word_tokens:BTreeMap<Ixx,Vec<String>>,
    pub fra_word_tokens:BTreeMap<Ixx,Vec<String>>,

}

/*
#[derive(Debug)]
pub enum WordsAsTokensLang {
    Eng(WordsAsTokens),
    Fra(WordsAsTokens)
}
*/

impl Debug for TokenN {
    fn fmt(&self, f: &mut Formatter ) -> fmt::Result {
        write!(f, "\nToken:\n  As indices:  {:?} \n  As string:  {}\n"
               , self.flattened_to_index
               , self.flattened_to_string)
    }
}



pub struct CandidatesForMergeN {
    pub eng_pairs:HashMap<(Ind,Ind),Quant>,
    pub fra_pairs:HashMap<(Ind,Ind),Quant>,

}

/*
pub enum CandidatesForMergeLang {
    Eng(CandidatesForMerge),
    Fra(CandidatesForMerge)
}
*/

pub struct MostFrequentPairN {
    pub eng_pair:(Ind,Ind),
    pub eng_pair_frequency:Quant,
    pub fra_pair:(Ind,Ind),
    pub fra_pair_frequency:Quant,

}

/*
pub enum MostFrequentPairLang {
    Eng(MostFrequentPair),
    Fra(MostFrequentPair)
}
*/

impl CandidatesForMergeN {
/*
    pub fn from_tokens_words_dynamic(word_indices:&BTreeMap<Ixx,Vec<Ind>>
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
            quant = *word_quantity.get(&word).unwrap();
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                *pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        CandidatesForMerge {
            pairs:pairs
        }

                
    } 
*/


    pub fn from_tokens_words_dynamic(dynamics:&TokensAndWordsDynamicsN) -> CandidatesForMergeN {
        let mut eng_pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();
        let mut fra_pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();

        let mut quant:Quant;
        let mut pair:(Ind,Ind);
        let mut size;

        for (word,collection) in &dynamics.eng_word_indices {
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            quant = *dynamics.eng_word_quantity.get(&word).unwrap();
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                *eng_pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        for (word,collection) in &dynamics.fra_word_indices {
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            quant = *dynamics.fra_word_quantity.get(&word).unwrap();
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                *fra_pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        CandidatesForMergeN {
            eng_pairs:eng_pairs,
            fra_pairs:fra_pairs

        }

                
    } 


/*
    pub fn from_word_vocab(index_word:&BTreeMap<Ixx,String>
                           ,words_n:&BTreeMap<Ixx,Vec<Ind>>
                           ,numbers:&BTreeMap<Ixx,Qxx>)  -> CandidatesForMerge {
        let mut pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();
        let mut quant:Quant;
        let mut pair:(Ind,Ind);
        let mut size;
        let mut collection;
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

*/

    pub fn from_word_vocab(vocab:&Vocab, indexation:&WordToIndexCollection)  -> CandidatesForMergeN {
        let mut eng_pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();
        let mut fra_pairs:HashMap<(Ind,Ind),Quant> = HashMap::new();

        let mut quant:Quant;
        let mut pair:(Ind,Ind);
        let mut size;
        let mut collection;

        for word in vocab.eng_index_word.keys() {
            collection = indexation.eng_words_n.get(&word).unwrap().to_vec();
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                quant = *vocab.eng_numbers.get(&word).unwrap();
                *eng_pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        for word in vocab.fra_index_word.keys() {
            collection = indexation.fra_words_n.get(&word).unwrap().to_vec();
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                quant = *vocab.fra_numbers.get(&word).unwrap();
                *fra_pairs.entry(pair).or_insert(quant)+=quant;
            }
        }


        CandidatesForMergeN {
            eng_pairs:eng_pairs,
            fra_pairs:fra_pairs,

        }
    }

/*
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
*/

    pub fn most_frequent_pair(&self) -> MostFrequentPairN {
        let closure = |pairs:&HashMap<(Ind,Ind),Quant>| {
            let res = max_key(pairs).expect("The vocabulary is to be not empty");
            (*res.0,*res.1)
        };

        let eng_max_pair = closure(&self.eng_pairs);
        let fra_max_pair = closure(&self.fra_pairs);

        MostFrequentPairN {
        eng_pair:eng_max_pair.0,    
        eng_pair_frequency:eng_max_pair.1,
        fra_pair:fra_max_pair.0,    
        fra_pair_frequency:fra_max_pair.1,

        }
    }
}


/*
impl CandidatesForMergeLang {

    pub fn from_tokens_words_dynamic(dynamics:&TokensAndWordsDynamicsLang) 
        -> CandidatesForMergeLang {
            match dynamics {
                TokensAndWordsDynamicsLang::Eng(x) => 
                    CandidatesForMergeLang
                    ::Eng(CandidatesForMerge
                          ::from_tokens_words_dynamic(&x.word_indices
                                                      ,&x.word_quantity)),

                TokensAndWordsDynamicsLang::Fra(y) => 
                    CandidatesForMergeLang
                    ::Fra(CandidatesForMerge
                          ::from_tokens_words_dynamic(&y.word_indices
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

*/

#[derive(Debug)]
pub struct TokensAndWordsDynamicsN {
// TODO is it possible to use &str instead of String ? with reference to token.flattened_to_string?
    pub eng_index_token:BTreeMap<Ind,TokenN>,
    pub eng_token_index:BTreeMap<String,Ind>,
    pub eng_word_quantity:BTreeMap<Ixx,Qxx>,
    pub eng_word_indices:BTreeMap<Ixx,Vec<Ind>>,
    pub fra_index_token:BTreeMap<Ind,TokenN>,
    pub fra_token_index:BTreeMap<String,Ind>,
    pub fra_word_quantity:BTreeMap<Ixx,Qxx>,
    pub fra_word_indices:BTreeMap<Ixx,Vec<Ind>>
    
}

pub struct TokensDynamicsAndEntropyN {
    pub eng_dyn_tokens:BTreeMap<Ind,Quant>,
    pub eng_entropy:f32,
    pub fra_dyn_tokens:BTreeMap<Ind,Quant>,
    pub fra_entropy:f32,
}

impl TokensAndWordsDynamicsN {
    pub fn new() -> TokensAndWordsDynamicsN {
        TokensAndWordsDynamicsN {
            eng_index_token:BTreeMap::new()
                ,eng_token_index:BTreeMap::new()
                ,eng_word_quantity:BTreeMap::new()
                ,eng_word_indices:BTreeMap::new()
                ,fra_index_token:BTreeMap::new()
                ,fra_token_index:BTreeMap::new()
                ,fra_word_quantity:BTreeMap::new()
                ,fra_word_indices:BTreeMap::new()

        }
    }

    pub fn initial_set_from_vocab(vocab_t:&VocabOfTokens
                                  ,vocab_w:&Vocab) -> TokensAndWordsDynamicsN {
        let mut eng_hsh_index:BTreeMap<Ind,TokenN> = BTreeMap::new();
        let mut eng_hsh_token:BTreeMap<String,Ind> = BTreeMap::new();
        let mut fra_hsh_index:BTreeMap<Ind,TokenN> = BTreeMap::new();
        let mut fra_hsh_token:BTreeMap<String,Ind> = BTreeMap::new();

// TODO rewrite to:  for (index,token) in index_token { .... }
        for index in &vocab_t.eng_index_token {
            let st = index.1.to_string();
            let token = TokenN {
                flattened_to_index:vec![*index.0],
                flattened_to_string:st.to_owned()
            };
// TODO check for containing the index key -> generate corresp behaviour
            eng_hsh_index.entry(*index.0).or_insert(token);
            eng_hsh_token.entry(st).or_insert(*index.0);
        }

// TODO rewrite to:  for (index,token) in index_token { .... }
        for index in &vocab_t.fra_index_token {
            let st = index.1.to_string();
            let token = TokenN {
                flattened_to_index:vec![*index.0],
                flattened_to_string:st.to_owned()
            };
// TODO check for containing the index key -> generate corresp behaviour
            fra_hsh_index.entry(*index.0).or_insert(token);
            fra_hsh_token.entry(st).or_insert(*index.0);
        }


        let mut eng_hsh_word_ics:BTreeMap<Ixx,Vec<Ind>> = BTreeMap::new();
        let mut fra_hsh_word_ics:BTreeMap<Ixx,Vec<Ind>> = BTreeMap::new();

        let mut char_index:Ind;
        let mut char_as_string:String;

        for (index,word) in &vocab_w.eng_index_word {
            let mut vec_of_indices:Vec<Ind>=Vec::new();
            for ch in word.chars() {
                char_as_string = ch.to_string();
                char_index = *vocab_t.eng_token_index.get(&char_as_string).unwrap();
                vec_of_indices.push(char_index);
            }
            eng_hsh_word_ics.entry(*index).or_insert(vec_of_indices);
        }

        for (index,word) in &vocab_w.fra_index_word {
            let mut vec_of_indices:Vec<Ind>=Vec::new();
            for ch in word.chars() {
                char_as_string = ch.to_string();
                char_index = *vocab_t.fra_token_index.get(&char_as_string).unwrap();
                vec_of_indices.push(char_index);
            }
            fra_hsh_word_ics.entry(*index).or_insert(vec_of_indices);
        }


        TokensAndWordsDynamicsN {
            eng_index_token:eng_hsh_index,
            eng_token_index:eng_hsh_token,
            eng_word_indices:eng_hsh_word_ics,
            eng_word_quantity:vocab_w.eng_numbers.to_owned(),
            fra_index_token:fra_hsh_index,
            fra_token_index:fra_hsh_token,
            fra_word_indices:fra_hsh_word_ics,
            fra_word_quantity:vocab_w.fra_numbers.to_owned()
        }
    }
 
    pub fn from_most_frequent_pair(&mut self,pair:&MostFrequentPairN) {
// for eng language
        let mut eng_to_index_left = self.eng_index_token
            .get(&pair.eng_pair.0)
            .unwrap()
            .flattened_to_index
            .to_vec();
        let mut eng_to_index_right = self.eng_index_token
            .get(&pair.eng_pair.1)
            .unwrap()
            .flattened_to_index
            .to_vec();
        eng_to_index_left.append(&mut eng_to_index_right);

        let mut eng_to_string_left = self.eng_index_token
            .get(&pair.eng_pair.0)
            .unwrap()
            .flattened_to_string
            .to_owned();
        let eng_to_string_right = self.eng_index_token
            .get(&pair.eng_pair.1)
            .unwrap()
            .flattened_to_string
            .to_owned();
        eng_to_string_left.push_str(&eng_to_string_right);
        let st = &eng_to_string_left.to_owned();

        let token = TokenN {flattened_to_index:eng_to_index_left,flattened_to_string:eng_to_string_left};
        
        let size = self.eng_index_token.keys().len();
        let new_index = size +1;
        if self.eng_index_token.contains_key(&new_index) {
            panic!("The new key already exist: {:?} ; panic!", new_index);
        }
        self.eng_index_token.insert(new_index,token);
// TODO what to do if "to_string_left" already exist ?
        if self.eng_token_index.contains_key(st) {
            panic!("The string key already exist: {:?}",st.to_string());
        }
        self.eng_token_index
            .entry(st.to_string())
            .or_insert(new_index);
// TODO find needed pair in vector of numbers and change the pair to a new number


        for (_index,vector) in self.eng_word_indices.iter_mut() {
            find_and_replace_pair(vector,&pair.eng_pair,&new_index);
        }
/*       
        self.eng_word_indices
        .iter()
        .map(|(_index,vector)| find_and_replace_pair(vector,&pair.eng_pair,&new_index).to_owned())
        .collect::<BTreeMap<Ixx,Vec<Ind>>>();   
*/
// for fra language
        let mut fra_to_index_left = self.fra_index_token
            .get(&pair.fra_pair.0)
            .unwrap()
            .flattened_to_index
            .to_vec();
        let mut fra_to_index_right = self.fra_index_token
            .get(&pair.fra_pair.1)
            .unwrap()
            .flattened_to_index
            .to_vec();
        fra_to_index_left.append(&mut fra_to_index_right);

        let mut fra_to_string_left = self.fra_index_token
            .get(&pair.fra_pair.0)
            .unwrap()
            .flattened_to_string
            .to_owned();
        let fra_to_string_right = self.fra_index_token
            .get(&pair.fra_pair.1)
            .unwrap()
            .flattened_to_string
            .to_owned();
        fra_to_string_left.push_str(&fra_to_string_right);
        let st = &fra_to_string_left.to_owned();

        let token = TokenN {flattened_to_index:fra_to_index_left,flattened_to_string:fra_to_string_left};
        
        let size = self.fra_index_token.keys().len();
        let new_index = size +1;
        if self.fra_index_token.contains_key(&new_index) {
            panic!("The new key already exist: {:?} ; panic!", new_index);
        }
        self.fra_index_token.insert(new_index,token);
// TODO what to do if "to_string_left" already exist ?
        if self.fra_token_index.contains_key(st) {
            panic!("The string key already exist: {:?}",st.to_string());
        }
        self.fra_token_index
            .entry(st.to_string())
            .or_insert(new_index);
// TODO find needed pair in vector of numbers and change the pair to a new number
/*        
        self.fra_word_indices
        .iter_mut()
        .map(|(_index,vector)| find_and_replace_pair(vector,&pair.fra_pair,&new_index))
        .collect();    
*/
        for (_index,vector) in self.fra_word_indices.iter_mut() {
            find_and_replace_pair(vector,&pair.fra_pair,&new_index);
        }


    }


     pub fn word_as_strings_collection(&self) -> WordsAsTokensN {
        let mut eng_map = BTreeMap::<Ixx,Vec<String>>::new();
        let mut fra_map = BTreeMap::<Ixx,Vec<String>>::new();

        for (ixx,collection) in &self.eng_word_indices {
            let mut substrings_collection = vec![];
            for ind in collection {
                substrings_collection
                    .push(self.eng_index_token
                          .get(&ind)
                          .unwrap()
                          .flattened_to_string
                          .to_owned());
            }
            eng_map.insert(*ixx, substrings_collection);
        }

        for (ixx,collection) in &self.fra_word_indices {
            let mut substrings_collection = vec![];
            for ind in collection {
                substrings_collection
                    .push(self.fra_index_token
                          .get(&ind)
                          .unwrap()
                          .flattened_to_string
                          .to_owned());
            }
            fra_map.insert(*ixx, substrings_collection);
        }


        WordsAsTokensN {
            eng_word_tokens:eng_map,
            fra_word_tokens:fra_map
        }
    }

/*
    fn word_as_strings_collection(&self) -> WordsAsTokens {
        let mut map = BTreeMap::<Ixx,Vec<String>>::new();
        for (ixx,collection) in &self.word_indices {
            let mut substrings_collection = vec![];
            for ind in collection {
                substrings_collection
                    .push(self.index_token
                          .get(&ind)
                          .unwrap()
                          .flattened_to_string
                          .to_owned());
            }
            map.insert(*ixx, substrings_collection);
        }
        WordsAsTokens {
            word_tokens:map
        }
    }
*/

    pub fn tokens_vocab_and_entropy(&self) -> TokensDynamicsAndEntropyN {
        let mut eng_tokens_distribution = BTreeMap::<Ind,Quant>::new();
        let mut fra_tokens_distribution = BTreeMap::<Ind,Quant>::new();
        let mut word_factor; 

        for (ixx,vec) in &self.eng_word_indices {
           word_factor = self.eng_word_quantity.get(&ixx).unwrap();
            for ind in vec {
                *eng_tokens_distribution
                    .entry(*ind)
                    .or_insert(*word_factor)+=word_factor;
            }
        }

        for (ixx,vec) in &self.fra_word_indices {
           word_factor = self.fra_word_quantity.get(&ixx).unwrap();
            for ind in vec {
                *fra_tokens_distribution
                    .entry(*ind)
                    .or_insert(*word_factor)+=word_factor;
            }
        }

// entropy calculation
        let mut sum:f32 = 0.0;
        let mut eng_entropy:f32 = 0.0;
        let mut fra_entropy:f32 = 0.0;


        for (_key,value) in &eng_tokens_distribution {
            sum += *value as f32;
       }
        for (_key,value) in &eng_tokens_distribution {
            let f = *value as f32/sum;
            eng_entropy -= f*f.log2();
        }

        for (_key,value) in &fra_tokens_distribution {
            sum += *value as f32;
       }
        for (_key,value) in &fra_tokens_distribution {
            let f = *value as f32/sum;
            fra_entropy -= f*f.log2();
        }


        TokensDynamicsAndEntropyN {
            eng_dyn_tokens:eng_tokens_distribution,
            eng_entropy:eng_entropy,
            fra_dyn_tokens:fra_tokens_distribution,
            fra_entropy:fra_entropy,

        }
    }   
}

/*
    pub fn tokens_vocab_and_entropy(&self) -> (BTreeMap<Ind,Quant>, f32) {
        let mut tokens_distribution = BTreeMap::<Ind,Quant>::new();
        let mut word_factor; 
        for (ixx,vec) in &self.word_indices {
           word_factor = self.word_quantity.get(&ixx).unwrap();
            for ind in vec {
                *tokens_distribution
                    .entry(*ind)
                    .or_insert(*word_factor)+=word_factor;
            }
        }

        let mut sum:f32 = 0.0;
        let mut entropy:f32 = 0.0;
        for (_key,value) in &tokens_distribution {
            sum += *value as f32;
       }
        for (_key,value) in &tokens_distribution {
            let f = *value as f32/sum;
            entropy -= f*f.log2();
        }

        (tokens_distribution,entropy)

    }   
*/
/*
#[derive(Debug)]
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

    pub fn word_as_strings_collection(&self) -> WordsAsTokensLang {
        match self {
            TokensAndWordsDynamicsLang::Eng(x) => 
                WordsAsTokensLang::Eng(x.word_as_strings_collection()),
            TokensAndWordsDynamicsLang::Fra(x) => 
                WordsAsTokensLang::Fra(x.word_as_strings_collection()),
        }
    }

    pub fn tokens_vocab_and_entropy(&self) -> (BTreeMap<Ind,Quant>, f32) {
        match self {
            TokensAndWordsDynamicsLang::Eng(x) => x.tokens_vocab_and_entropy(),
            TokensAndWordsDynamicsLang::Fra(x) => x.tokens_vocab_and_entropy(),
                                                                            
        }
    }
}
*/


#[derive(Serialize, Deserialize, Debug)]
pub struct SentencesAsIndicesDynamicsN {
    pub eng_words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub eng_words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub eng_sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>,
    pub fra_words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub fra_words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub fra_sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>

}

/*
#[derive(Serialize, Deserialize, Debug)]
pub enum SentencesAsIndicesDynamicsLang {
    Eng(SentencesAsIndicesDynamics),
    Fra(SentencesAsIndicesDynamics)
}
*/

impl SentencesAsIndicesDynamicsN {
    pub fn new() -> SentencesAsIndicesDynamicsN {
        SentencesAsIndicesDynamicsN {
        eng_words_as_indices:BTreeMap::new(),
        eng_words_as_token_indices:BTreeMap::new(),
        eng_sentence_flattened_to_token_indices:BTreeMap::new(),
        fra_words_as_indices:BTreeMap::new(),
        fra_words_as_token_indices:BTreeMap::new(),
        fra_sentence_flattened_to_token_indices:BTreeMap::new(),
        }
    }

    pub fn initial_from_sentences_and_indices(sentences:&SentencesAsIndices) 
        -> SentencesAsIndicesDynamicsN {
        let mut eng_sents_flatten = BTreeMap::new();
        let mut fra_sents_flatten = BTreeMap::new();

        for (ixs,collection) in &sentences.eng_word_as_tokens_n {
            eng_sents_flatten
                .insert(*ixs,collection.iter().flat_map(|x| x.to_owned()).collect());
        }

        for (ixs,collection) in &sentences.fra_word_as_tokens_n {
            fra_sents_flatten
                .insert(*ixs,collection.iter().flat_map(|x| x.to_owned()).collect());
        }

        SentencesAsIndicesDynamicsN {
        eng_words_as_indices:sentences.eng_word_as_index.to_owned(),
        eng_words_as_token_indices:sentences.eng_word_as_tokens_n.to_owned(),
        eng_sentence_flattened_to_token_indices:eng_sents_flatten,
        fra_words_as_indices:sentences.fra_word_as_index.to_owned(),
        fra_words_as_token_indices:sentences.fra_word_as_tokens_n.to_owned(),
        fra_sentence_flattened_to_token_indices:fra_sents_flatten,

        }
    }  
/*
    pub fn from_tokens_words_dynamic(&mut self, word_indices:&BTreeMap<Ixx,Vec<Ind>>) { // dynamics.word_indices 
        let mut wd;
        for (ixs,word) in self.words_as_indices.to_owned() {
            wd = word
                .iter()
                .map(|z| word_indices.get(z).unwrap().to_owned())
                .collect::<Vec<Vec<Ind>>>();
            self.words_as_token_indices.insert(ixs,wd);
        }

        for (ixs,word) in self.words_as_indices.to_owned() {
        let mut wdd:Vec<Ind> = Vec::new();

            for ind in word.iter() {
                wdd.append(&mut word_indices.get(ind).unwrap().to_owned()); 
            }
            self.sentence_flattened_to_token_indices.insert(ixs,wdd.to_owned());
        }
    }
*/

    pub fn from_tokens_words_dynamic(&mut self, dynamics:&TokensAndWordsDynamicsN) { // dynamics.word_indices 
        let mut wd;
        for (ixs,word) in self.eng_words_as_indices.to_owned() {
            wd = word
                .iter()
                .map(|z| dynamics.eng_word_indices.get(z).unwrap().to_owned())
                .collect::<Vec<Vec<Ind>>>();
            self.eng_words_as_token_indices.insert(ixs,wd);
        }

        for (ixs,word) in self.eng_words_as_indices.to_owned() {
        let mut wdd:Vec<Ind> = Vec::new();

            for ind in word.iter() {
                wdd.append(&mut dynamics.eng_word_indices.get(ind).unwrap().to_owned()); 
            }
            self.eng_sentence_flattened_to_token_indices.insert(ixs,wdd.to_owned());
        }

        for (ixs,word) in self.fra_words_as_indices.to_owned() {
            wd = word
                .iter()
                .map(|z| dynamics.fra_word_indices.get(z).unwrap().to_owned())
                .collect::<Vec<Vec<Ind>>>();
            self.fra_words_as_token_indices.insert(ixs,wd);
        }

        for (ixs,word) in self.fra_words_as_indices.to_owned() {
        let mut wdd:Vec<Ind> = Vec::new();

            for ind in word.iter() {
                wdd.append(&mut dynamics.fra_word_indices.get(ind).unwrap().to_owned()); 
            }
            self.fra_sentence_flattened_to_token_indices.insert(ixs,wdd.to_owned());
        }


    }
}

