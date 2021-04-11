use std::fmt::{self,Debug,Formatter};
use super::*;

// the struct is keeping 2 Vectors with sentences : one for englich 
// and another for the french pairs

pub struct SentencesForTranslation {
    pub eng:Vec<String>,
    pub fra:Vec<String>,
    pub eng_as_words:Vec<Vec<String>>,
    pub fra_as_words:Vec<Vec<String>>,
    pub eng_max_words_sentence:Ixx,
    pub fra_max_words_sentence:Ixx,
    pub size:Ixs,

// to be populated after vocab of words formation
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
            eng:eng,
            fra:fra,
            size:size,
            eng_as_words:Vec::new(),
            fra_as_words:Vec::new(),
            eng_max_words_sentence:0,
            fra_max_words_sentence:0,
            
        }

    }

    pub fn from_sentence(&mut self) {
        let mut res_eng:Vec<Vec<String>>=Vec::with_capacity(self.size);
        let mut res_fra:Vec<Vec<String>>=Vec::with_capacity(self.size);
        let mut max_eng = 0;
        let mut max_fra = 0;
        for sentence in &self.eng {
//            let mut eng_collector:Vec<String> = Vec::with_capacity(MAX_WORDS_IN_SENTENCE_SOURCE);
            let mut eng_collector:Vec<String> = Vec::new();

            let mut eng_counter = 0;
            for word in sentence.trim().split_whitespace(){
                eng_collector.push(word.to_owned());
                eng_counter +=1;
            }
            res_eng.push(eng_collector);
            if eng_counter > max_eng {
                max_eng = eng_counter;
            }
        }
    

        for sentence in &self.fra {
//            let mut fra_collector:Vec<String>= Vec::with_capacity(MAX_WORDS_IN_SENTENCE_TARGET);
            let mut fra_collector:Vec<String>= Vec::new();

            let mut fra_counter = 0;
            for word in sentence.trim().split_whitespace(){
                fra_collector.push(word.to_owned());
                fra_counter+=1;
            }
            res_fra.push(fra_collector);
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

pub struct SentencesInIndices {
    pub eng_words_as_indices:Vec<Vec<Ixx>>,
    pub fra_words_as_indices:Vec<Vec<Ixx>>,
}

impl SentencesInIndices {
    pub fn new() -> SentencesInIndices {
        SentencesInIndices {
        eng_words_as_indices:Vec::new(),
        fra_words_as_indices:Vec::new(),
        }
    }   
//TODO
/*
    pub fn from_word_vocab(&mut self, vocab:&Vocab, sentences:&SentencesForTranslation) {
        for ind in sentences.eng_as_words {
            let mut sent_as_indices_eng = Vec::with_capacity(sentences.eng_as_words.len());
//            let mut sent_as_indices_fra = Vec::with_capacity(sentences.fra_as_words.len());
            sent_as_indices_eng = self.eng_words_as_indices[ind]
                .iter()
                .map(|x| vocab.eng_word_index.get(x).unwrap()).collect();
            self.eng_words_as_indices.insert(ind,sent_as_indices_eng);

        }

    
    }
*/
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
    pub as_text:HashMap<Ixs,(String,String)>,
    pub as_words:HashMap<Ixs,(Vec<String>,Vec<String>)>,
    pub as_indices:HashMap<Ixs,(Vec<Vec<Ixx>>,Vec<Vec<Ixx>>)>,
}

impl PairsForTranslation {

    pub fn from_sentences(sentences:&SentencesForTranslation) -> PairsForTranslation { 
            let mut hsh = HashMap::with_capacity(sentences.size);
            for i in 0..sentences.eng.len() {
                hsh
                    .insert(i,(sentences.eng[i].to_owned(),sentences.fra[i].to_owned()));
            }

            PairsForTranslation {
                as_text:hsh
                    ,as_words:HashMap::with_capacity(sentences.size)
                    ,as_indices:HashMap::with_capacity(sentences.size)
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

