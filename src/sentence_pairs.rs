use std::fmt::{self,Debug,Formatter};
use super::*;

// the struct is keeping 2 Vectors with sentences : one for englich 
// and another for the french pairs

pub struct SentencesForTranslation {
    pub eng:Vec<String>,
    pub fra:Vec<String>,
    pub size:Ixs,
}

impl SentencesForTranslation {
    pub fn from_corpus(corpus:&CorpusAsString) -> SentencesForTranslation{
        let mut eng:Vec<String>=vec![];
        let mut fra:Vec<String>=vec![];
        for sub in corpus.processed.lines() {
            let mut it = sub.split("\t");
            eng.push(it.next().unwrap().to_owned());
            fra.push(it.next().unwrap().to_owned());

        } 

        let size = eng.len();
        if size != fra.len() {
            panic!("Quantity of source sentences is not same as for target. Panic! ");
        }

        SentencesForTranslation {
            eng:eng, fra:fra, size:size
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
            pairs.push(TranslationPair::from_sentences(&sentences.eng[i], &sentences.fra[i]));
        }

        TranslationPairs {pairs:pairs}

    }
}

// map of translation pairs, in a pair a first is source, 
// the second is target
pub struct PairsForTranslation {
    as_text:HashMap<Ixs,(String,String)>,
    as_words:HashMap<Ixs,(Vec<String>,Vec<String>)>
}

impl PairsForTranslation {
    pub fn from_sentences(sentences:&SentencesForTranslation) 
        -> PairsForTranslation { 
            let mut hsh = HashMap::with_capacity(sentences.eng.len());
            for i in 0..sentences.eng.len() {
                hsh.insert(i,(sentences.eng[i].to_owned(),sentences.fra[i].to_owned()));
            }

            PairsForTranslation {as_text:hsh,as_words:HashMap::new()}
        }
//TODO
    pub fn from_vords_vocab(&mut self,vocab:&Vocab) {
        return ()
    }
}

