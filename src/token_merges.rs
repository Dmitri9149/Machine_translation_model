use super::*;
use std::collections::HashMap;

pub struct CandidatesForMerge {
    pub eng_pairs:HashMap<(Ind,Ind),Quant>,
    pub fra_pairs:HashMap<(Ind,Ind),Quant>

}

impl CandidatesForMerge {

    pub fn new() -> CandidatesForMerge {
        CandidatesForMerge {
            eng_pairs:HashMap::new(),
            fra_pairs:HashMap::new()
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

    impl pub fn most_frequent_pair(hash:&CandidatesForMerge) -> (Ind,Ind) {

    }
}

