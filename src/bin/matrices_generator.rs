// take data from tokens_generator (sentencesAsIndicesDynamics) which are in 
// the form of sentence -> (token_indices) like [1,56,390] where the nubers are 
// indices for initial and newly generated tokens
// the data are transformed to the Matrix to be used for matrix transformations in 
// machine translation 

//use serde_json::{Result, Value};
use translationlib::*;
use std::fs::File;
use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;
use ndarray::prelude::*;

static NUMBER_TOKENS:Ind = 573;
static NUMBER_PAIRS:Ixs = 185583;



/*
fn main() -> Result<(),Box<dyn std::error::Error>>{

    Ok(())
}
*/

// started development from here 
// https://github.com/serde-rs/serde/issues/1195
fn main() {
    let json_file_path = Path::new("data/data.json");
    let json_file_str = read_to_string(json_file_path).expect("file not found");
    let start = Instant::now();
    // use instead of from_reader
    let sentences:SentencesAsIndicesDynamicsLang = serde_json::from_str(&json_file_str).expect("error while reading json");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("The sentences in initial form: {:?}", sentences);

    let mut pairs_tokens_matrix:Array2<f32> = Array::zeros((NUMBER_PAIRS,NUMBER_TOKENS));

    match sentences {
        SentencesAsIndicesDynamicsLang::Eng(x)=> 
            for (ixs,collection) in x.sentence_flattened_to_token_indices.iter() {
                for ind in collection.iter() {
                    pairs_tokens_matrix[[ixs,ind]] = 1.;
                }
            },
            
        _ => (),
    }

    println!("Array ! :{:?}", pairs_tokens_matrix);

}

/*
pub struct SentencesAsIndicesDynamics {
    pub words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>
}
*/
