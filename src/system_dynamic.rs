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

/*
pub struct FraToken {
//    eng_token:HashMap<Ind,Vec<Ind>>,
//    fra_token:HashMap<Ind,Vec<Ind>>,
    fra_flattened_to_index:Vec<Ind>,
    fra_flattened_to_string:String,
}

pub struct EngToken {
//    eng_token:HashMap<Ind,Vec<Ind>>,
//    fra_token:HashMap<Ind,Vec<Ind>>,
    eng_flattened_to_index:Vec<Ind>,
    eng_flattened_to_string:String,
}
*/
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

/*
impl MostFrequentPair {
    pub fn from_merge_candidates(candidates:&CandidatesForMerge) -> MostFrequentPair {
        let closure = |pairs:&HashMap<(Ind,Ind),Quant>| {
            let res = max_key(pairs).expect("The vocabulary is to be not empty");
            (*res.0,*res.1)
        };

        let eng_max_pair = closure(&candidates.eng_pairs);
        let fra_max_pair = closure(&candidates.fra_pairs);
        MostFrequentPair {
        eng:eng_max_pair.0,    
        fra:fra_max_pair.0,
        eng_frequency:eng_max_pair.1,
        fra_frequency:fra_max_pair.1
        }
    }
/*
    pub fn new_token(pair:&MostFrequentPair,)
*/
}
*/
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
/*
    pub fn most_frequent_pair(&self) -> MostFrequentPair {
        let closure = |pairs:&HashMap<(Ind,Ind),Quant>| {
            let res = max_key(pairs).expect("The vocabulary is to be not empty");
            (*res.0,*res.1)
        };

        let eng_max_pair = closure(&self.eng_pairs);
        let fra_max_pair = closure(&self.fra_pairs);
        MostFrequentPair {
        eng:eng_max_pair.0,    
        fra:fra_max_pair.0,
        eng_frequency:eng_max_pair.1,
        fra_frequency:fra_max_pair.1
        }
    }
*/
/* the function return the key with biggest value
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<(&K,&V)>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
//        .map(|(k, v)| (k,v))
}
*/


//    pub fn new_token()
/*
// calculate the most frequent pair of consequtive tokens in words of VocabStage
    pub fn key_max(&self) -> (String, String) {
        let res = max_key(&self.pairs).expect("The vocabulary is to be not empty");
        (res.0.to_string(),res.1.to_string())
    }
*/
}

impl CandidatesForMergeLang {
    pub fn from_word_vocab(vocab:&Vocab, collection:&WordToIndexCollection,lang:Lang) 
        -> CandidatesForMergeLang {
            match lang {
                Eng => 
                    CandidatesForMergeLang::Eng(CandidatesForMerge::from_word_vocab(
                            &vocab.eng_index_word
                            ,&collection.eng_words_n
                            ,&vocab.eng_numbers
                                                           )),

                Fra => 
                    CandidatesForMergeLang::Eng(CandidatesForMerge::from_word_vocab(
                            &vocab.fra_index_word
                            ,&collection.fra_words_n
                            ,&vocab.fra_numbers
                                                           )),

            }
        }
}


pub struct NewToken {
    new_token:Token,
}
/*
// keep records of all new + initial ('letters') tokens and 
// the indices of the tokens
*/
pub struct TokensDynamic {
    index_token:HashMap<Ind,Token>,
}
/*
impl TokensDynamic {
    pub fn new() -> TokensDynamic {
        TokensDynamic {
            eng_index_token:HashMap::new(),
            fra_index_token:HashMap::new()
        }
    }

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
}
*/



