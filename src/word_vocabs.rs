// vocabulary for eng and fra corpus , on the level of words
use std::collections::BTreeMap;
use super::*;

// is used for calculating quantity of words
//pub type Qxx = u32;
// is used for the indexing (numbering) of words
// pub type Ixx = u32;

pub struct Vocab {
// the list of all words (may with possible repeats) which we can get by splitting all 
// sentences (usually we split on white space)
    pub eng_set:Vec<String>,
    pub fra_set:Vec<String>,
// (word, quantity of the words) in our corpus
    pub eng_words:BTreeMap<String,Qxx>,
    pub fra_words:BTreeMap<String,Qxx>,
// (number, quantity of the number) number here is the number equivalent of a word  (it's index)
    pub eng_numbers:BTreeMap<Ixx,Qxx>,
    pub fra_numbers:BTreeMap<Ixx,Qxx>,
// from number equivalent to word representation
    pub eng_index_word:BTreeMap<Ixx,String>,
    pub fra_index_word:BTreeMap<Ixx,String>,
// from word to its number representation
    pub eng_word_index:BTreeMap<String,Ixx>,
    pub fra_word_index:BTreeMap<String,Ixx>,
// number of words in fra or eng 
    pub eng_words_total:Ixx,
    pub fra_words_total:Ixx,
}

impl Vocab {
    pub fn new() -> Self { Vocab {
        eng_set:vec![],
        fra_set:vec![],
        eng_words:BTreeMap::new(),
        fra_words:BTreeMap::new(),
        eng_numbers:BTreeMap::new(),
        fra_numbers:BTreeMap::new(),
        eng_index_word:BTreeMap::new(),
        fra_index_word:BTreeMap::new(),
        eng_word_index:BTreeMap::new(),
        fra_word_index:BTreeMap::new(),
        eng_words_total:0,
        fra_words_total:0,
        }   
    }
//TODO
/*
    pub fn from_sentences(&mut self
                          ,pairs:&mut PairsForTranslation
                          ,sentences:&SentencesForTranslation) {
        let size = vector_sentences.eng.len();
        let size_fra = vector_sentences.fra.len();
        if size != size_fra {
            panic!("Quantity of source sentences is different from quantity of target sentences. Panic! ");
        }
        let mut res_eng:Vec<String>=Vec::with_capacity(size_eng);
        let mut res_fra:Vec<String>=Vec::with_capacity(size_fra);

    }
*/

    pub fn list_of_words(&mut self, vector_sentences:&SentencesForTranslation) {
        let size_eng = vector_sentences.eng.len();
        let size_fra = vector_sentences.fra.len();
        let mut res_eng:Vec<String>=Vec::with_capacity(size_eng);
        let mut res_fra:Vec<String>=Vec::with_capacity(size_fra);
        for sentence in &vector_sentences.eng_as_words {
            for word in sentence{
                res_eng.push(word.to_owned());
            }
        }

        for sentence in &vector_sentences.fra_as_words {
            for word in sentence {
                res_fra.push(word.to_owned());
            }

        }

        self.eng_set=res_eng;
        self.fra_set=res_fra;
        
    }

/*
    pub fn list_of_words(&mut self, vector_sentences:&SentencesForTranslation) {
        let size_eng = vector_sentences.eng.len();
        let size_fra = vector_sentences.fra.len();
        let mut res_eng:Vec<String>=Vec::with_capacity(size_eng);
        let mut res_fra:Vec<String>=Vec::with_capacity(size_fra);
        let mut max_eng = 0;
        let mut max_fra = 0;
        for sentence in &vector_sentences.eng {
            let mut eng_counter = 0;
            for word in sentence.trim().split_whitespace(){
                res_eng.push(word.to_owned());
                eng_counter +=1;
            }
            if eng_counter > max_eng {
                max_eng = eng_counter;
            }
        }

        for sentence in &vector_sentences.fra {
            let mut fra_counter = 0;
            for word in sentence.trim().split_whitespace(){
                res_fra.push(word.to_owned());
                fra_counter+=1;
            }

            if fra_counter > max_fra {
                max_fra = fra_counter;
            }
        }

        self.eng_set=res_eng;
        self.fra_set=res_fra;
        self.eng_max_words_sentence=max_eng;
        self.fra_max_words_sentence=max_fra;
        
    }
*/
    pub fn words_and_quantity(&mut self) {
        for word in self.eng_set.iter() {
            let count = self
                .eng_words
                .entry(word.to_owned())
                .or_insert(0);
            *count+=1;
        }
        for word in self.fra_set.iter() {
            let count = self
                .fra_words
                .entry(word.to_owned())
                .or_insert(0);
            *count+=1;
        }

        self.eng_words_total = self.eng_words.keys().len() as Ixx;
        self.fra_words_total = self.fra_words.keys().len() as Ixx;

    }
// unique numbers are attached to every word in vocab and this is stored in hash structures
// so we can go from word to its number representation and back
// the total quantity of eng, fra words is also calculated here
    pub fn word_to_index(&mut self) {
        let mut index:Ixx = 0;
        for word in &self.eng_words {
            self.eng_word_index
                .insert(word.0.to_owned(), index);
            self.eng_words_total+=1;
            index+=1;

        }

        let mut index:Ixx = 1;
        for word in &self.fra_words {
            self.fra_word_index
                .insert(word.0.to_owned(), index);
            self.fra_words_total+=1;
            index+=1;

        }

    }
    pub fn index_to_word(&mut self) {
        for word in &self.eng_word_index {
            self.eng_index_word
                .insert(*word.1,word.0.to_owned());
        }
        for word in &self.fra_word_index {
            self.fra_index_word
                .insert(*word.1, word.0.to_owned());
        }

    }

    pub fn index_quantity(&mut self) {
        let closure = |words:&BTreeMap<String,Qxx>
            ,word_index:&BTreeMap<String,Ixx>
            ,numbers:&mut BTreeMap<Ixx,Qxx>| {
                for (word,quant) in words {
                    numbers.insert(word_index.get(word).unwrap().to_owned(),*quant);
                }              
            };
        closure(&self.eng_words,&self.eng_word_index,&mut self.eng_numbers);
        closure(&self.fra_words,&self.fra_word_index,&mut self.fra_numbers);
    }
}
// words are represented as numbers (index) and String (collection of chars)
// words may be represented as colletion of token indixes in the struct , 

pub struct WordToIndexCollection {
// the pairs in the Vectors: 
// word as string is paired with collection of token indixes 
// word as number is paired with the collection of token indixes 
    pub eng_words_s:BTreeMap<String,Vec<Ind>>,
    pub fra_words_s:BTreeMap<String,Vec<Ind>>,
    pub eng_words_n:BTreeMap<Ixx,Vec<Ind>>,
    pub fra_words_n:BTreeMap<Ixx,Vec<Ind>>,
    pub eng_word_max_length:Ixw,
    pub fra_word_max_length:Ixw,

}


impl WordToIndexCollection {
    pub fn new() -> WordToIndexCollection {
        WordToIndexCollection {
        eng_words_s:BTreeMap::new(),
        fra_words_s:BTreeMap::new(),
        eng_words_n:BTreeMap::new(),
        fra_words_n:BTreeMap::new(),
        eng_word_max_length:0,
        fra_word_max_length:0,
        }
    }

    pub fn from_word_vocab(&mut self, word_vocab:&Vocab,token_vocab:&VocabOfTokens) {
        let closure = |words_s:&mut BTreeMap<String,Vec<Ind>>
            ,words_n:&mut BTreeMap<Ixx,Vec<Ind>>
            ,words:&BTreeMap<String,Quant>
            ,word_index:&BTreeMap<String,Ixx>
            ,token_index:&BTreeMap<String,Ind>
            ,max_length:&mut Ixw | {
                *max_length =0;
                for (word,_) in words {
                    let words_iter = word.chars();
// approximate length of word in chars by the length is bytes
                    let collection:&mut Vec<Ind> = &mut Vec::with_capacity(word.len());
                    let mut current_length:Ixw = 0;
                    for ch in words_iter {
                        collection.push(*token_index.get(&ch.to_string()).unwrap());
                        current_length+=1;
                    }
                    words_s.insert(word.to_string(),collection.to_vec());
                    words_n.insert(*word_index.get(word).unwrap(),collection.to_vec());
                    if current_length > *max_length {
                        *max_length = current_length;
                    }
                }
            };

        closure(&mut self.eng_words_s
                ,&mut self.eng_words_n
                ,&word_vocab.eng_words
                ,&word_vocab.eng_word_index
                ,&token_vocab.eng_token_index
                ,&mut self.eng_word_max_length);
        closure(&mut self.fra_words_s
                ,&mut self.fra_words_n
                ,&word_vocab.fra_words
                ,&word_vocab.fra_word_index
                ,&token_vocab.fra_token_index
                ,&mut self.fra_word_max_length);




    }
}
