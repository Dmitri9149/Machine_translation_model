// read text file which contains translation pairs and save it 
// as one big string, some preprocessing at the level of the 
// whole string is possible (like add spaces between ? mark and end of sentences)
//
pub struct CorpusAsString {
    corpus:String,

}


impl CorpusAsAsting {

// build by reading a file
    pub fn corpus_from_file(path: &str) -> CorpusAsString {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        CorpusAsString {
            corpus: contents.clone()
        }
    }
}
