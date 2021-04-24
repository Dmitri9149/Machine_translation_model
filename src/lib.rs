use std::collections::HashMap;
use std::collections::BTreeMap;

mod translation_corpus;
mod sentence_pairs;
mod word_vocabs;
mod tokens_vocab;
mod system_dynamic;
mod sentence_dynamic;

pub mod word_dynamics;

pub use crate::translation_corpus::{CorpusAsString};
pub use crate::sentence_pairs::{SentencesForTranslation
    ,SentencesAsIndices
    ,TranslationPair
    ,TranslationPairs};
pub use crate::word_vocabs::{Vocab,WordToIndexCollection};
pub use tokens_vocab::{VocabOfTokens};
pub use system_dynamic::{CandidatesForMerge
    ,CandidatesForMergeLang
    ,MostFrequentPair
    ,MostFrequentPairLang
    ,TokensAndWordsDynamics
    ,TokensAndWordsDynamicsLang
    ,Token
    ,WordsAsTokens
    ,WordsAsTokensLang
    ,SentencesAsIndicesDynamicsLang
    ,SentencesAsIndicesDynamics
};
pub use sentence_dynamic::{SentenceAsWords
    ,SentenceAsWordsLang
    ,WordsAndSentenceDynamicsLang
    ,WordsAndSentenceDynamics
    ,CandidatesForMergeLang as OtherCandidatesForMergeLang
    ,MostFrequentPairLang as OtherMostFrequentPairLang
    ,MostFrequentPair as OtherMostFrequentPair

//    ,Idiom
//    ,CandidatesForMerge
//    ,CandidatesForMergeLang
};

pub use word_dynamics::dynamics::{CandidatesForMergeN
    ,MostFrequentPairN
    ,TokensAndWordsDynamicsN
    ,TokenN
    ,WordsAsTokensN
    ,SentencesAsIndicesDynamicsN
};


// the index for sentences numbering
pub type Ixs = usize;
// indexation of tokens 
pub type Ind = usize;
// for tokens quantity
pub type Quant = u64;
// indexation of words
pub type Ixx= usize;
// for words quantity
pub type Qxx = u64;
// Lang varints 
pub enum Lang {
    Eng,
    Fra
} 
// for word length 
pub type Ixw=u16;

//static SENTENCES_NUMBER:Ixs = 190000;
// max quantity of words in a sentence
//static MAX_WORDS_IN_SENTENCE_SOURCE:Ixx = 30;
//static MAX_WORDS_IN_SENTENCE_TARGET:Ixx = 30;


pub fn replace_chars_to_char(input:&str, aa:&str, b:char) -> String {
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        if aa.contains(c) {
            output.push(b);
        } else {
            output.push(c);
        }
    }

    output
}

// separate a punctuation symbol from the list 'st' by " " from both sides 
pub fn separate_punctuation(strng:&str, st:&str) -> String{
    let mut res=strng.to_owned();
    for ch in st.chars() {
        res = res
            .replace(&ch.to_string(), &[" ", &ch.to_string()].join(""))
            .to_owned();
    }
    res
}

/*
// the function return the key with biggest value for HashMap
fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}
*/

// the function return the key with biggest value
pub fn max_key<K, V>(a_hash_map: &HashMap<K, V>) -> Option<(&K,&V)>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
//        .map(|(k, v)| (k,v))
}
// take vector of numbers find pais which are equal to pair-parameter and change the pair to new
// (number)
pub fn find_and_replace_pair(vect:&mut Vec<Ind>,pair:&(Ind,Ind),new:&Ind) {
    let size = vect.len();
    if size == 0 {
        panic!("The vector of Ind is empty !!");
    } else if size == 1 {
        return ()
    }
    let mut pointer = 0;
    let mut collector:Vec<Ind> = Vec::new();
    let mut counter = 0;
    let mut flag = false;
    while counter < size-1 {
        if (vect[counter],vect[counter+1]) == (pair.0,pair.1) {
            collector.append(&mut vect[pointer..counter].to_vec());
            collector.push(*new);
            counter+=2;
            pointer = counter;
            flag = true;

            continue;
        } 
        counter+=1;
    }

    if flag == false {
        return ()
    } else {
        collector.append(&mut vect[pointer..size].to_vec());
    }

    *vect = collector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(0,0);
    }

    #[test]
    fn find_and_replace_pair_b() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector1 = vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100];
        let mut vector2 = vec![3];
        let mut vector3 = vec![3,100,3,100,3,100,3,100,1,2,3,3,100];
        let mut vector4 = vec![3,100];

        find_and_replace_pair(&mut vector1,&pair,&new);
        find_and_replace_pair(&mut vector2,&pair,&new);
        find_and_replace_pair(&mut vector3,&pair,&new);
        find_and_replace_pair(&mut vector4,&pair,&new);

        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], vector1);
        assert_eq!(vec![3], vector2);
        assert_eq!(vec![777,777,777,777,1,2,3,777], vector3);
        assert_eq!(vec![777], vector4);


        let pair = (57,62);
        let new = 91;
        let mut vector5 = vec![71, 63, 66, 66, 73, 57, 62, 55];
        find_and_replace_pair(&mut vector5,&pair,&new);
        assert_eq!(vec![71,63,66,66,73,91,55], vector5);
       
    }

    #[test]
    #[should_panic]
    fn find_and_change_pairs_in_vector_with_panic() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector = vec![];
        find_and_replace_pair(&mut vector,&pair,&new);
    }

    #[test]
    fn find_and_replace_in_iter_mut() {

        let pair = (3,100);
        let new = 777;
        let mut map:BTreeMap<Ixx,Vec<Ind>> = BTreeMap::new();
        map.insert(0,vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100]);
        map.insert(1,vec![3]);
        map.insert(2,vec![3,100,3,100,3,100,3,100,1,2,3,3,100]);
        map.insert(3,vec![3,100]);

        for (_index,vector) in map.iter_mut() {
            find_and_replace_pair(vector,&pair,&new);
        }

        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], *map.get(&0).unwrap());
        assert_eq!(vec![3], *map.get(&1).unwrap());
        assert_eq!(vec![777,777,777,777,1,2,3,777], *map.get(&2).unwrap());
        assert_eq!(vec![777], *map.get(&3).unwrap()); 
    }
}

















