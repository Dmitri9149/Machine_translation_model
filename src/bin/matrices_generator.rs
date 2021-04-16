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
}

