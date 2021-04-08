use std::collections::HashMap;

mod translation_corpus;
mod sentence_pairs;
mod word_vocabs;
mod tokens_vocab;
mod system_dynamic;

pub use crate::translation_corpus::{CorpusAsString};
pub use crate::sentence_pairs::{SentencesForTranslation,TranslationPair,
TranslationPairs};
pub use crate::word_vocabs::{Vocab,WordToIndexCollection};
pub use tokens_vocab::{VocabOfTokens};
pub use system_dynamic::{CandidatesForMerge
    ,CandidatesForMergeLang
    ,MostFrequentPair
    ,MostFrequentPairLang
    ,TokensAndWordsDynamics
    ,TokensAndWordsDynamicsLang
    ,Token
};

// indexation of tokens 
pub type Ind = usize;
// for tokens quantity
pub type Quant = u32;
// indexation of words
pub type Ixx= usize;
// for words quantity
pub type Qxx = u32;
// Lang varints 
pub enum Lang {
    Eng,
    Fra
} 


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
pub fn find_and_change_pair(vect:&mut Vec<Ind>,pair:&(Ind,Ind),new:&Ind) -> Vec<Ind> {
    let size = vect.len();
    if size == 0 {
//        println!("The vector of Ind is empty !!");
        return vect.to_vec()
    } else if size == 1 {
        return vect.to_vec()
    }
    let mut pointer = 0;
    let mut collector:Vec<Ind> = Vec::new();
    let mut counter = 0;
    let mut pointers:Vec<(Ind,Ind)>=Vec::new();
    while counter < size {
        if (vect[counter],vect[counter+1]) == (pair.0,pair.1) {
            pointers.push((pointer,counter));
            counter+=2;
            pointer = counter;
//            println!("pointers {:?}",pointers);
            continue;
        }

        counter+=1;
    }
    if pointers.len() !=0 {
        for (pointer,counter) in pointers {
            collector.append(&mut vect[pointer..counter].to_vec());
            collector.push(*new);
        }
    return collector
    }
    return vect.to_vec()
}
// same as for the function above , but the vectro is changed in place
pub fn find_and_change_in_place_pair(vect:&mut Vec<Ind>,pair:&(Ind,Ind),new:&Ind) {
    let size = vect.len();
    if size == 0 {
        panic!("The vector of Ind is empty !!");
    } else if size == 1 {
        return ()
    }
    let mut pointer = 0;
    let mut collector:Vec<Ind> = Vec::new();
    let mut counter = 0;
    let mut pointers:Vec<(Ind,Ind)>=Vec::new();
    while counter < size-1 {
        if (vect[counter],vect[counter+1]) == (pair.0,pair.1) {
            pointers.push((pointer,counter));
            counter+=2;
            pointer = counter;
//            println!("pointers {:?}",pointers);
            continue;
        }

        counter+=1;
    }
    if pointers.len() !=0 {
        for (pointer,counter) in pointers {
            collector.append(&mut vect[pointer..counter].to_vec());
            collector.push(*new);
        }
    *vect=collector;
    }
    return ()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(0,0);
    }

    #[test]
    fn find_and_change_pairs_in_vector() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector = vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100];
        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], find_and_change_pair(&mut vector,&pair,&new));
    }

    #[test]
    fn find_and_change_pairs_in_vector_in_place() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector1 = vec![1,2,100,3,100,3,100,5,78,39,1,2,3,3,100];
        let mut vector2 = vec![3];
        let mut vector3 = vec![3,100,3,100,3,100,3,100,1,2,3,3,100];
        let mut vector4 = vec![3,100];


        find_and_change_in_place_pair(&mut vector1,&pair,&new);
        find_and_change_in_place_pair(&mut vector2,&pair,&new);
        find_and_change_in_place_pair(&mut vector3,&pair,&new);
        find_and_change_in_place_pair(&mut vector4,&pair,&new);
        assert_eq!(vec![1,2,100,777,777,5,78,39,1,2,3,777], vector1);
        assert_eq!(vec![3], vector2);
        assert_eq!(vec![777,777,777,777,1,2,3,777], vector3);
        assert_eq!(vec![777], vector4);
    }

    #[test]
    #[should_panic]
    fn find_and_change_pairs_in_vector_with_panic() {
        println!("In test function!");
        let pair = (3,100);
        let new = 777; 
        let mut vector = vec![];
        find_and_change_in_place_pair(&mut vector,&pair,&new);
    }


}

















