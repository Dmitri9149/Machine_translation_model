// take data from tokens_generator (sentencesAsIndicesDynamics) which are in 
// the form of sentence -> (token_indices) like [1,56,390] where the numbers are 
// indices for the newly generated tokens
// tokens here are totally generated from vocabulaty of words, from initial characters
// the crate makes renumbering of tokens into contigues index (without possible gaps in old index)
// the number of tokens is calculated 
// the data are saved in json file using serde

//use serde_json::{Result, Value};
use translationlib::*;
use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;
use ndarray::*;
use ndarray_linalg::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::{self,Debug,Formatter};
use serde::{Serialize, Deserialize};

// the index for sentences numbering
pub type Ixs = usize;
// indexation of tokens 
pub type Ind = usize;
//  tokens quantity
pub type Quant = u32;
// indexation of words
pub type Ixx= usize;
// for words quantity
pub type Qxx = u32; 
// for word length 
pub type Ixw=u16;


fn main() -> Result<(),Box<dyn std::error::Error>> {
    let json_file_path = Path::new("data/sentence_as_indices_dynamics.json");
    let json_file_str = read_to_string(json_file_path).expect("file ..sentences...not found");
    let json_file_path_1 = Path::new("data/vocab.json");
    let json_file_str_1 = read_to_string(json_file_path_1).expect("file ...vocab..not found");

    let start = Instant::now();
    // use instead of from_reader
    let sentences:SentencesAsIndicesDynamicsN = serde_json
        ::from_str(&json_file_str)
        .expect("error while reading json");

    let vocab:Vocab = serde_json
        ::from_str(&json_file_str_1)
        .expect("error while reading json");
    
//    println!("The sentences in initial form: {:?}", sentences);
    let tokens_renumbering = GeneratedTokensCounting::from_sentences_as_indices_dynamics(&sentences);
    let mut words_renumbering = GeneratedWordsCounting::new();
    words_renumbering.from_initial_vocab(&vocab);

    println!("The number of eng tokens: {}\n", &tokens_renumbering.eng_number_of_tokens);
    println!("The number of fra tokens: {}\n", &tokens_renumbering.fra_number_of_tokens);

    ::serde_json::to_writer(&File::create("data/renumbering/tokens_renumbering.json")?, &tokens_renumbering)?;
    ::serde_json::to_writer(&File::create("data/renumbering/words_renumbering.json")?, &words_renumbering)?;

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
