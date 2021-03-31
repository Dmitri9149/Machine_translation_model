// the struct is keeping 2 Vectors with sentences : one for englich 
// and another for the french pairs

use super::*;
pub struct SentencesForTranslation {
    pub eng:Vec<String>,
    pub fra:Vec<String>
}

impl SentencesForTranslation {
    pub fn from_corpus(corpus:&CorpusAsString) -> SentencesForTranslation{
        let mut eng:Vec<String>=vec!["".to_owned()];
        let mut fra:Vec<String>=vec!["".to_owned()];
        for sub in corpus.processed.lines() {
            let mut it = sub.split("\t");
            eng.push(it.next().unwrap().to_owned());
            fra.push(it.next().unwrap().to_owned());

        }

        SentencesForTranslation {
            eng:eng, fra:fra
        }

    }

}
