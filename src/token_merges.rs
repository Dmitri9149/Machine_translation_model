use super::*;
use std::collections::HashMap;

pub struct CandidatesForMerge {
    pub eng_pairs:HashMap<(Ind,Ind),Quant>,
    pub fra_pairs:HashMap<(Ind,Ind),Quant>

}

pub struct MostFrequentPair {
    eng:(Ind,Ind),
    fra:(Ind,Ind),
    eng_frequency:Quant,
    fra_frequency:Quant
}

impl CandidatesForMerge {

    pub fn new() -> CandidatesForMerge {
        CandidatesForMerge {
            eng_pairs:HashMap::new(),
            fra_pairs:HashMap::new(),
        }
    }

    pub fn from_word_vocab(&mut self,vocab:&Vocab,word_collection:&WordToIndexCollection) {
        let mut size = word_collection.eng_words_n.len();
        let mut closure = |pairs:&mut HashMap<(Ind,Ind),Quant>
            ,vocab:&Vocab
            ,word_collection:&WordToIndexCollection| {
            let mut quant:Quant;
            let mut collection:Vec<Ind> = vec![];
            let mut pair:(Ind,Ind);
            for word in vocab.eng_index_word.keys() {
                collection = word_collection.eng_words_n.get(&word).unwrap().to_vec();
                size = collection.len();
                    if size == 0 {
                        panic!("from CandidatesForMerge: collection has 0 length, breack");
                    } else if size ==1 {
                        continue
                    }
                for i in 0..size-1 {
                    pair = (collection[i],collection[i+1]);
                    quant = *vocab.eng_numbers.get(&word).unwrap();
                    *pairs.entry(pair).or_insert(quant)+=quant;
                }
            } 
        };

        closure(&mut self.eng_pairs,&vocab,&word_collection);
        closure(&mut self.fra_pairs,&vocab,&word_collection);

    }

    pub fn most_frequent_pair(&self) -> MostFrequentPair {
        let closure = |pairs:&HashMap<(Ind,Ind),Quant>| {
            let res = max_key(pairs).expect("The vocabulary is to be not empty");
            res
        };

        let eng_max_pair = closure(&self.eng_pairs);
        let fra_max_pair = closure(&self.fra_pairs);
        MostFrequentPair {
        eng:*eng_max_pair.0,    
        fra:*fra_max_pair.0,
        eng_frequency:*eng_max_pair.1,
        fra_frequency:*fra_max_pair.1
        }
    }

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





