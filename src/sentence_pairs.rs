use std::fmt::{self,Debug,Formatter};
use std::collections::BTreeMap;
use super::*;

// the struct is keeping 2 Vectors with sentences : one for englich 
// and another for the french pairs

pub struct SentencesForTranslation {
    pub eng:BTreeMap<Ixs,String>,
    pub fra:BTreeMap<Ixs,String>,
    pub eng_as_words:BTreeMap<Ixs,Vec<String>>,
    pub fra_as_words:BTreeMap<Ixs,Vec<String>>,
    pub eng_max_words_sentence:Ixx,
    pub fra_max_words_sentence:Ixx,
    pub size:Ixs,

// to be populated after vocab of words formation
}

impl SentencesForTranslation {
    pub fn from_corpus(corpus:&CorpusAsString) -> SentencesForTranslation{
        let mut eng=BTreeMap::new();
        let mut fra=BTreeMap::new();
        let mut ixs = 0;
        for sub in corpus.processed.lines() {
            let mut it = sub.split("\t");
            eng.insert(ixs,it.next().unwrap().to_owned());
            fra.insert(ixs,it.next().unwrap().to_owned());
            ixs+=1;

        } 

        let size = eng.len();
        if size != fra.len() {
            panic!("Quantity of source sentences is not same as for target. Panic! ");
        }

        SentencesForTranslation {
            eng:eng,
            fra:fra,
            size:size,
            eng_as_words:BTreeMap::new(),
            fra_as_words:BTreeMap::new(),
            eng_max_words_sentence:0,
            fra_max_words_sentence:0,
            
        }

    }

    pub fn from_sentence(&mut self) {
        let mut res_eng:BTreeMap<Ixs,Vec<String>>=BTreeMap::new();
        let mut res_fra:BTreeMap<Ixs,Vec<String>>=BTreeMap::new();
        let mut max_eng = 0;
        let mut max_fra = 0;
        for (ind,sentence) in &self.eng {
//            let mut eng_collector:Vec<String> = Vec::with_capacity(MAX_WORDS_IN_SENTENCE_SOURCE);
            let mut eng_collector:Vec<String> = Vec::new();

            let mut eng_counter = 0;
            for word in sentence.trim().split_whitespace(){
                eng_collector.push(word.to_owned());
                eng_counter +=1;
            }
            res_eng.insert(*ind,eng_collector);
            if eng_counter > max_eng {
                max_eng = eng_counter;
            }
        }
    

        for (ind,sentence) in &self.fra {
//            let mut fra_collector:Vec<String>= Vec::with_capacity(MAX_WORDS_IN_SENTENCE_TARGET);
            let mut fra_collector:Vec<String>= Vec::new();

            let mut fra_counter = 0;
            for word in sentence.trim().split_whitespace(){
                fra_collector.push(word.to_owned());
                fra_counter+=1;
            }
            res_fra.insert(*ind,fra_collector);
            if fra_counter > max_fra {
                max_fra = fra_counter;
            }
        }

        self.eng_as_words=res_eng;
        self.fra_as_words=res_fra;
        self.eng_max_words_sentence=max_eng;
        self.fra_max_words_sentence=max_fra;


    }

}

pub struct SentencesAsIndices {
    pub eng_word_as_index:BTreeMap<Ixs,Vec<Ixx>>,
    pub fra_word_as_index:BTreeMap<Ixs,Vec<Ixx>>,
    pub eng_word_as_tokens_n:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub fra_word_as_tokens_n:BTreeMap<Ixs,Vec<Vec<Ind>>>
}

impl SentencesAsIndices {
    pub fn new() -> SentencesAsIndices {
        SentencesAsIndices {
        eng_word_as_index:BTreeMap::new(),
        fra_word_as_index:BTreeMap::new(),
        eng_word_as_tokens_n:BTreeMap::new(),
        fra_word_as_tokens_n:BTreeMap::new()

        }
    }   
//TODO

    pub fn from_word_vocab(&mut self, vocab:&Vocab, sentences:&SentencesForTranslation) {
        for (ind, sentence) in &sentences.eng_as_words {
            let mut sent_as_indices_eng = Vec::with_capacity(sentences.eng_as_words.get(&ind).unwrap().len());
            sent_as_indices_eng = sentence
                .iter()
                .map(|x| *vocab.eng_word_index.get(x).unwrap()).collect();
            self.eng_word_as_index.insert(*ind,sent_as_indices_eng);
        }

        for (ind, sentence) in &sentences.fra_as_words {
            let mut sent_as_indices_fra = Vec::with_capacity(sentences.fra_as_words.get(&ind).unwrap().len());
            sent_as_indices_fra = sentence
                .iter()
                .map(|x| *vocab.fra_word_index.get(x).unwrap()).collect();
            self.fra_word_as_index.insert(*ind,sent_as_indices_fra);
        }

    }

    pub fn from_word_as_tokens(&mut self,collections:&WordToIndexCollection) {

        for (ind,sentence) in &self.eng_word_as_index {
            let mut tokens= Vec::with_capacity(sentence.len());
            for word in sentence {
                tokens.push(collections.eng_words_n.get(&word).unwrap().to_owned())
            }

            self.eng_word_as_tokens_n
                .insert(*ind,tokens);
        }
        for (ind,sentence) in &self.fra_word_as_index {
            let mut tokens= Vec::with_capacity(sentence.len());
            for word in sentence {
                tokens.push(collections.fra_words_n.get(&word).unwrap().to_owned())
            }

            self.fra_word_as_tokens_n
                .insert(*ind,tokens);
        }


    }
}


// eng and fra sentences as a pair in one struct 
pub struct TranslationPair {
    pub eng:String,
    pub fra:String
}

impl TranslationPair {
    pub fn from_sentences(eng:&str,fra:&str) -> TranslationPair{
        TranslationPair {eng:eng.to_owned(), fra:fra.to_owned()}
    }
}

// Vector of TranslationPairs 
pub struct TranslationPairs {
    pub pairs:Vec<TranslationPair>

}

impl Debug for TranslationPair {
    fn fmt(&self, f: &mut Formatter ) -> fmt::Result {
        write!(f, "\nPair:\neng:  {} \nfra:  {}\n", self.eng, self.fra)
    }
}


impl TranslationPairs {
// construct it from SentencesForTranslation
    pub fn from_sentences(sentences:&SentencesForTranslation) -> TranslationPairs{
        let size_eng = sentences.eng.len();
        let size_fra = sentences.fra.len();
        if size_eng != size_fra {
            panic!("Numbers of source and target sentences are different ! Panic.");
        }

        let size = size_eng;
        
        let mut pairs:Vec<TranslationPair>= Vec::with_capacity(size_eng);
        for i in 0..size {
            pairs.push(TranslationPair::from_sentences(&sentences.eng.get(&i).unwrap()
                                                       , &sentences.fra.get(&i).unwrap()));
        }

        for key in sentences.eng.keys() {
            pairs.push(TranslationPair::from_sentences(&sentences.eng.get(&key).unwrap()
                                                       , &sentences.fra.get(&key).unwrap()));
            
        }

        TranslationPairs {pairs:pairs}

    }
}

// map of translation pairs, in a pair a first is source, 
// the second is target
pub struct PairsForTranslation {
    pub as_text:BTreeMap<Ixs,(String,String)>,
    pub as_words:BTreeMap<Ixs,(Vec<String>,Vec<String>)>,
    pub as_tokens:BTreeMap<Ixs,(Vec<Vec<Ind>>,Vec<Vec<Ind>>)>,
}

impl PairsForTranslation {

    pub fn from_sentences(sentences:&SentencesForTranslation) -> PairsForTranslation { 
            let mut hsh_as_text = BTreeMap::new();
            let mut hsh_as_words = BTreeMap::new();
//            let mut hsh_as_tokens = BTreeMap::new();
            let size = sentences.size;
            if size == 0 {
                panic!("Size of sentenses for translation collection is zero ! Panic!");
            }
            for i in sentences.eng.keys() {
                hsh_as_text
                    .insert(*i,(sentences.eng.get(i).unwrap().to_owned()
                               ,sentences.fra.get(i).unwrap().to_owned()));
                hsh_as_words
                    .insert(*i,(sentences.eng_as_words.get(i).unwrap().to_owned()
                               ,sentences.fra_as_words.get(i).unwrap().to_owned()));
                

            }

            PairsForTranslation {
                as_text:hsh_as_text
                    ,as_words:hsh_as_words
                    ,as_tokens:BTreeMap::new()
            }
    }
//TODO
/*
    pub fn from_vords_vocab(&mut self,vocab:&Vocab) {
        for (ind) in self.eng_as_words.iter {
            let mut sent_as_indices_eng = Vec::new();
            let mut sent_as_indices_fra = Vec::new();
            (sent_as_indices_eng, sent_as_indices_fra) = (self.as_words[i].0
                .iter()
                .map(|x| vocab.eng_word_index.get(x).unwrap()).collect();
        }
        self.eng_as_index.insert(ind,sent_as_indices);
    }
*/
}

