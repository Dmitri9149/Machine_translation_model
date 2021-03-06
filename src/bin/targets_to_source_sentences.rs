// take data from tokens_generator (sentencesAsIndicesDynamics) which are in 
// the form of sentence -> (token_indices) like [1,56,390] where the nubers are 
// indices for initial and newly generated tokens
// tokens here are totally generated from vocabulaty of words, from characters
// the data are transformed to the Matrix to be used for matrix transformations in 
// machine translation 

//use serde_json::{Result, Value};
/*
use std::fs::File;
use ndarray::*;
use ndarray_linalg::*;

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt::{self,Display,Debug,Formatter};
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};
*/
use serde::{Serialize, Deserialize};
use std::time::Instant;
use std::path::Path;
use std::fs::read_to_string;

use translationlib::{NOWORD
    ,word_dynamics::dynamics::SentencesAsIndicesDynamicsN
    ,TARGET_SENTENCE_MAX_LEN
    ,SOURCE_SENTENCE_MAX_LEN};

use translationlib::targets_to_sentences::targets_to_lengths::{TargetsPosition
    ,TargetWordsToSentences
    ,TargetWordsToSentencesBuilder
    ,Config};
use translationlib::probability::length_likelihood::{
    TargetLengths
    ,TargetWordsToSentenceLengths
    ,PositionalTargetWordsCount
    ,PositionalTargetWordsProbability
    ,PositionalWordsPredictor
};



fn main()  -> Result<(),Box<dyn std::error::Error>> {
    let json_file_path_sentences = Path::new("data/matrices_generator/sentences_as_indices_dynamics.json");
    let json_file_str_sentences = read_to_string(json_file_path_sentences).expect("file not found");
//    let json_file_path_renumbering = Path::new("data/renumbering/tokens_renumbering.json");
//    let json_file_str_renumbering = read_to_string(json_file_path_renumbering).expect("file not found");
/*
    let json_file_path_words_renum = Path::new("data/renumbering/words_renumbering.json");
    let json_file_str_words_renum = read_to_string(json_file_path_words_renum).expect("file not found");
*/

    let start = Instant::now();
    // use instead of from_reader
    let sentences:SentencesAsIndicesDynamicsN = serde_json
        ::from_str(&json_file_str_sentences)
        .expect("error while reading json with sentences");

    let target_words_to_sentences = TargetWordsToSentencesBuilder::new()
        .no_word(&NOWORD)
        .build(&sentences)?;

    let mut target_words_to_sentence_lengths = TargetWordsToSentenceLengths
        ::from_words_to_sentences(&target_words_to_sentences,&sentences);
    target_words_to_sentence_lengths.lengths_likelihood(&SOURCE_SENTENCE_MAX_LEN);

    let mut target_words_count = PositionalTargetWordsCount::new();
    target_words_count.from_target_words_to_sentence_lengths(&target_words_to_sentence_lengths);

    let target_words_probability = PositionalTargetWordsProbability
        ::new(&target_words_count);

    let mut words_predictor = PositionalWordsPredictor
        ::from_frequency_and_likelihood(&target_words_probability
                                         ,&target_words_to_sentence_lengths);
    words_predictor.predict();



// print sentences 
//    println!("The sentences in initial form: {:?}", &sentences);

// print max lengths of sentences
println!("Target sentence max length: {}\n", &sentences.target_sentence_max_len);
println!("Source sentence max length: {}\n", &sentences.source_sentence_max_len);

//print targets...to sentences   and targets...to lengths


/*
println!("The targets_words_to_sentences: {:?}\n",&target_words_to_sentences
         .words_sentences_collections
         .get(&0)
         .unwrap());
*/

println!("Words predictor for first word {:?}\n",&target_words_to_sentence_lengths
         .words_to_lengths_collections
         .get(&0)
         .unwrap()
         .lengths_likelihood
         );


println!("The targets_words_to_sentence_lengths {:?}\n",&words_predictor
         .words_from_lengths_predictions
         .get(&0)
         .unwrap()
         .words_scores
         .get(&2)
         .unwrap()
         );

for i in 0..TARGET_SENTENCE_MAX_LEN {
    println!("Predicted word position in target sentence (0 index is for first word): {}\n", &i);
    println!("The targets_words_to_sentence_lengths {:?}\n",&words_predictor
             .words_from_lengths_predictions
             .get(&(i as u16))
             .unwrap()
             .best_word
             );
}

/*
    ::serde_json::to_writer(&File::create("data/matrices_generator/words_to_sentences.json")?, &words_to_sentences)?;
    ::serde_json::to_writer(&File::create("data/matrices_generator/words_to_lengths.json")?, &words_to_lengths)?;

*/
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

