use super::*;

/*
#[derive(Copy,Clone)]
pub enum PrefixSuffix {
    Begin,
    Middle,
    End,
    Nn
}
*/

// #[derive(Copy,Clone)]
pub struct CandidatesForMerge {
    pair:(Ixx,Ixx),
//    kind:PrefixSuffix,
//    mass_in_chars:Quant,
//    mass_in_tokens:u16

}

impl CandidatesForMerge {
/*    
    pub fn from_data(n1:&Ixx,n2:&Ixx,kn:&PrefixSuffix,in_chars:&Quant,in_tokens:&u16) -> CandidatesForMerge {
        CandidatesForMerge {
            pair:(*n1,*n2),
            kind:*kn,
            mass_in_chars:*in_chars,
            mass_in_tokens:*in_tokens
        }
*/

    pub fn from_data(n1:&Ixx,n2:&Ixx) -> CandidatesForMerge {
        CandidatesForMerge {
            pair:(*n1,*n2)
        }

    }
}

pub struct MergedTokens {
    pub fn from_vocab(&vocab:VocabOfTokens)
}


