// system in dynamics, merge of most frequent tokens, 
// dynamic changes in vocabulary because of new tokens,
// bookkeeping of new tokens 
use super::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::{self,Debug,Formatter};


// keep the records of the flattened tokens as 
// list of indices or as String
pub struct Idiom {
    pub flattened_to_index:Vec<Ixx>,
    pub flattened_to_collection:Vec<String>,
}

#[derive(Debug)]
pub struct SentenceAsWords {
    pub sentence_idioms:BTreeMap<Ixs,Vec<Vec<String>>>,
}

#[derive(Debug)]
pub enum SentenceAsWordsLang {
    Eng(SentenceAsWords),
    Fra(SentenceAsWords)
}


impl Debug for Idiom {
    fn fmt(&self, f: &mut Formatter ) -> fmt::Result {
        write!(f, "\nIdiom:\n  As indices:  {:?} \n  As collection of string:  {:?}\n"
               , self.flattened_to_index
               , self.flattened_to_collection)
    }
}



pub struct CandidatesForMerge {
    pub pairs:HashMap<(Ixx,Ixx),Qxx>,
}

pub enum CandidatesForMergeLang {
    Eng(CandidatesForMerge),
    Fra(CandidatesForMerge)
}

pub struct MostFrequentPair {
    pub pair:(Ixx,Ixx),
    pub pair_frequency:Qxx,
}

impl Debug for MostFrequentPair {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\nMostFrequentPair:  ({}{})\nFrequency:  {}\n"
               ,self.pair.0,self.pair.1,self.pair_frequency)
    }
}

pub enum MostFrequentPairLang {
    Eng(MostFrequentPair),
    Fra(MostFrequentPair)
}



impl CandidatesForMerge {
    pub fn from_words_sentence_dynamic(sentence_indices:&BTreeMap<Ixs,Vec<Ixx>>) -> CandidatesForMerge {
        let mut pairs:HashMap<(Ixx,Ixx),Qxx> = HashMap::new();
        let mut quant:Qxx;
        let mut pair:(Ixx,Ixx);
        let mut size;
        for (sentence,collection) in sentence_indices {
            size = collection.len();
                if size == 0 {
                    panic!("from CandidatesForMerge: collection has 0 length, breack");
                } else if size ==1 {
                    continue
                }
            quant = 1;
            for i in 0..size-1 {
                pair = (collection[i],collection[i+1]);
                *pairs.entry(pair).or_insert(quant)+=quant;
            }
        }

        CandidatesForMerge {
            pairs:pairs
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

    pub fn most_frequent_pair(&self) -> MostFrequentPair {
        let closure = |pairs:&HashMap<(Ixx,Ixx),Qxx>| {
            let res = max_key(pairs).expect("The vocabulary for pairs is to be not empty! Panic!");
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

    pub fn from_words_sentence_dynamic(dynamics:&WordsAndSentenceDynamicsLang) 
        -> CandidatesForMergeLang {
            match dynamics {
                WordsAndSentenceDynamicsLang::Eng(x) => 
                    CandidatesForMergeLang
                    ::Eng(CandidatesForMerge
                          ::from_words_sentence_dynamic(&x.sentence_indices)),

                WordsAndSentenceDynamicsLang::Fra(y) => 
                    CandidatesForMergeLang
                    ::Fra(CandidatesForMerge
                          ::from_words_sentence_dynamic(&y.sentence_indices)),
            }
    }

/*
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
*/
}

impl MostFrequentPair {
    pub fn get_as_words(&self,vocab:&BTreeMap<Ixx,String>) -> ((String,String),Qxx) {
        ((vocab.get(&self.pair.0).unwrap().to_owned(),vocab.get(&self.pair.1).unwrap().to_owned()),
        self.pair_frequency)
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


#[derive(Debug)]
pub struct WordsAndSentenceDynamics {
    pub index_idiom:BTreeMap<Ixx,Idiom>,
    pub idiom_index:BTreeMap<Vec<String>,Ixx>,
    pub sentence_indices:BTreeMap<Ixs,Vec<Ixx>>
    
}

impl WordsAndSentenceDynamics {
    pub fn new() -> WordsAndSentenceDynamics {
        WordsAndSentenceDynamics {
            index_idiom:BTreeMap::new()
                ,idiom_index:BTreeMap::new()
                ,sentence_indices:BTreeMap::new()
        }
    }

    pub fn initial_from_sentences(index_word:&BTreeMap<Ixx,String>
                                  ,word_index:&BTreeMap<String,Ixx>
                                  ,sentence_as_indices:&BTreeMap<Ixs,Vec<Ixx>>) -> WordsAndSentenceDynamics {
        let mut hsh_index_idiom:BTreeMap<Ixx,Idiom> = BTreeMap::new();
        let mut hsh_idiom_index:BTreeMap<Vec<String>,Ind> = BTreeMap::new();
        for (ind,word) in index_word {
            let idiom = Idiom {
                flattened_to_index:vec![*ind],
                flattened_to_collection:vec![word.to_owned()]
            };
            hsh_index_idiom.entry(*ind).or_insert(idiom);
            hsh_idiom_index.entry(vec![word.to_owned()]).or_insert(*ind);
        }
        WordsAndSentenceDynamics {
            index_idiom:hsh_index_idiom
            ,idiom_index:hsh_idiom_index
            ,sentence_indices:sentence_as_indices.to_owned()
        }
    }

 
    pub fn from_most_frequent_pair(&mut self,pair:&MostFrequentPair) {
        let mut to_index_left = self.index_idiom.get(&pair.pair.0).unwrap().flattened_to_index.to_vec();
        let mut to_index_right = self.index_idiom.get(&pair.pair.1).unwrap().flattened_to_index.to_vec();
        to_index_left.append(&mut to_index_right);

        let mut to_collection_left = self.index_idiom.get(&pair.pair.0).unwrap().flattened_to_collection.to_owned();
        let mut to_collection_right = self.index_idiom.get(&pair.pair.1).unwrap().flattened_to_collection.to_owned();
        to_collection_left.append(&mut to_collection_right);
        let collection = &to_collection_left.to_owned();

        let idiom = Idiom {flattened_to_index:to_index_left,flattened_to_collection:to_collection_left};
        
        let size = self.index_idiom.keys().len();
        let new_index = size +1;
        if self.index_idiom.contains_key(&new_index) {
            panic!("The new key already exist: {:?} ; panic!", new_index);
        }
        self.index_idiom.insert(new_index,idiom);
// TODO what to do if "to_collection_left" already exist ?
        if self.idiom_index.contains_key(collection) {
            panic!("The string key already exist: {:?}",collection.to_owned());
        }
        self.idiom_index.entry(collection.to_owned()).or_insert(new_index);
// TODO find needed pair in vector of numbers and change the pair to a new number
        
        self.sentence_indices
        .iter_mut()
        .map(|(_index,vector)| find_and_replace_pair(vector,&pair.pair,&new_index))
        .collect()    
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

    fn sentence_as_words_collection(&self) -> SentenceAsWords {
        let mut map = BTreeMap::<Ixs,Vec<Vec<String>>>::new();
        for (ixs,collection) in &self.sentence_indices {  // sentence_indices -> Vec<Ixx>
            let mut substrings_collection = vec![];
            for ind in collection {
// index_idiom -> BTreeMap<Ixx,Idiom> Idiom -> Vec<Ixx> and Vec<String>
                substrings_collection
                    .push(self.index_idiom  
                          .get(&ind)
                          .unwrap()
                          .flattened_to_collection   // Vec<String>
                          .to_vec());
            }
            map.insert(*ixs, substrings_collection);
        }
        SentenceAsWords {
            sentence_idioms:map
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
}

#[derive(Debug)]
pub enum WordsAndSentenceDynamicsLang {
    Eng(WordsAndSentenceDynamics),
    Fra(WordsAndSentenceDynamics)
}


impl WordsAndSentenceDynamicsLang {

    pub fn new(lang:Lang) -> WordsAndSentenceDynamicsLang {
        match lang {
            Lang::Eng => WordsAndSentenceDynamicsLang::Eng(WordsAndSentenceDynamics::new()),
            Lang::Fra => WordsAndSentenceDynamicsLang::Fra(WordsAndSentenceDynamics::new()),
        }
    }

    pub fn initial_from_sentences(lang:Lang
                                  ,vocab:&Vocab
                                  ,sentences:&SentencesAsIndices) -> WordsAndSentenceDynamicsLang{
        match lang {
            Lang::Eng  => WordsAndSentenceDynamicsLang
                ::Eng(WordsAndSentenceDynamics::initial_from_sentences(&vocab.eng_index_word
                                                            ,&vocab.eng_word_index
                                                            ,&sentences.eng_word_as_index)),
            Lang::Fra  => WordsAndSentenceDynamicsLang
                ::Fra(WordsAndSentenceDynamics::initial_from_sentences(&vocab.fra_index_word
                                                            ,&vocab.fra_word_index
                                                            ,&sentences.fra_word_as_index)),
        }
    }


    pub fn from_most_frequent_pair(&mut self,pair:&MostFrequentPairLang) {
        match self {
            WordsAndSentenceDynamicsLang::Eng(x) => 
                match pair {
                    MostFrequentPairLang::Eng(y) => x.from_most_frequent_pair(y),
                    _ => panic!("TokensDynamicLang error: Source...Target language data inconsistency"),
                }
            WordsAndSentenceDynamicsLang::Fra(x) => 
                match pair {
                    MostFrequentPairLang::Fra(y) => x.from_most_frequent_pair(y),
                    _ => panic!("TokensDynamicLang error: Source...Target language data inconsistency"),
                }
        }
    }

    pub fn sentence_as_words_collection(&self) -> SentenceAsWordsLang {
        match self {
            WordsAndSentenceDynamicsLang::Eng(x) => 
                SentenceAsWordsLang::Eng(x.sentence_as_words_collection()),
            WordsAndSentenceDynamicsLang::Fra(x) => 
                SentenceAsWordsLang::Fra(x.sentence_as_words_collection()),
        }
    }
}


