// vocabulary for eng and fra corpus , on the level of words
use std::collections::HashMap;
use super::*;

// is used for calculating quantity of words
pub type Qxx = u32;
// is used for the indexing (numbering) of words
pub type Ixx = u32;

pub struct Vocab {
// the list of all words (may repeat) which we can get by splitting all 
// sentences (usually we split on white space)
    pub eng_set:Vec<String>,
    pub fra_set:Vec<String>,
// (word, quantity of the words) in our corpus
    pub eng_words:HashMap<String,Qxx>,
    pub fra_words:HashMap<String,Qxx>,
// (number, quantity of the number) number here is the number equivalent of a word  (it's index)
    pub eng_numbers:HashMap<Ixx,Qxx>,
    pub fra_numbers:HashMap<Ixx,Qxx>,
// from number equivalent to word representation
    pub eng_index_word:HashMap<Ixx,String>,
    pub fra_index_word:HashMap<Ixx,String>,
// from word to its number representation
    pub eng_word_index:HashMap<String,Ixx>,
    pub fra_word_index:HashMap<String,Ixx>,
// number of words in fra or eng 
    pub eng_words_total:Ixx,
    pub fra_words_total:Ixx,

}

impl Vocab {
    pub fn new() -> Self { Vocab {
        eng_set:vec![],
        fra_set:vec![],
        eng_words:HashMap::new(),
        fra_words:HashMap::new(),
        eng_numbers:HashMap::new(),
        fra_numbers:HashMap::new(),
        eng_index_word:HashMap::new(),
        fra_index_word:HashMap::new(),
        eng_word_index:HashMap::new(),
        fra_word_index:HashMap::new(),
        eng_words_total:0,
        fra_words_total:0,
    }
    }

    pub fn list_of_words(&mut self, vector_sentences:&SentencesForTranslation) {
        let size = vector_sentences.eng.len();
        let mut res_eng:Vec<String>=Vec::with_capacity(size);
        let mut res_fra:Vec<String>=Vec::with_capacity(size);
        for sentence in &vector_sentences.eng {
            for word in sentence.trim().split_whitespace(){
                res_eng.push(word.to_owned());
            }
        }

        for sentence in &vector_sentences.fra {
            for word in sentence.trim().split_whitespace(){
                res_fra.push(word.to_owned());
            }
        }

        self.eng_set=res_eng;
        self.fra_set=res_fra;   
    }

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
    pub fn word_to_index(&mut self) {
        let mut index:Ixx = 0;
        for word in &self.eng_words {
            self.eng_word_index
                .insert(word.0.to_owned(), index);
            index+=1;

        }

        let mut index:Ixx = 1;
        for word in &self.fra_words {
            self.fra_word_index
                .insert(word.0.to_owned(), index);
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
        let closure = |words:&HashMap<String,Qxx>
            ,word_index:&HashMap<String,Ixx>
            ,numbers:&mut HashMap<Ixx,Qxx>| {
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
    eng_words_s:Vec<(String,Vec<Vec<Ind>>)>,
    fra_words_s:Vec<(String,Vec<Vec<Ind>>)>,
    eng_words_n:Vec<(Ixx,Vec<Vec<Ind>>)>,
    fra_words_n:Vec<(Ixx,Vec<Vec<Ind>>)>

}

impl WordToIndexCollection {
    pub fn from_word_vocab(&mut self, word_vocab:&Vocab,token_vocab:&VocabOfTokens) {
        let closure = |words_s:&mut Vec<(String,Vec<Vec<Ind>>)>
            ,words:&HashMap<String,Quant>
            ,token_index:&HashMap<String,Ind>| {
                for (word,_) in words {
                    let collection:&mut Vec<Vec<Ind>> = &mut Vec::new();

                    for ch in word.chars() {
                        let mut vec_char:Vec<Ind>= Vec::with_capacity(1);
                        vec_char.push(*token_index.get(&ch.to_string()).unwrap());
                        collection.push(vec_char);
                    }
                    words_s.push((word.to_string(),collection.to_vec()));
                }

            };

        closure(&mut self.eng_words_s,&word_vocab.eng_words,&token_vocab.eng_token_index);
        closure(&mut self.fra_words_s,&word_vocab.fra_words,&token_vocab.fra_token_index);




    }
}
