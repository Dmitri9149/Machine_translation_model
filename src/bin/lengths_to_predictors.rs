// take data from tokens_generator (sentencesAsIndicesDynamics) which are in 
// the form of sentence -> (token_indices) like [1,56,390] where the nubers are 
// indices for initial and newly generated tokens
// tokens here are totally generated from vocabulaty of words, from characters
// the data are transformed to the Matrix to be used for matrix transformations in 
// machine translation 

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
use std::fmt::{self,Display,Debug,Formatter};
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer,SerializeSeq, SerializeMap, SerializeStruct};

fn main()  -> Result<(),Box<dyn std::error::Error>> {
    let json_file_path_sentences = Path::new("data/matrices_generator/sentence_as_indices_dynamics.json");
    let json_file_str_sentences = read_to_string(json_file_path_sentences).expect("file not found");
    let json_file_path_renumbering = Path::new("data/renumbering/tokens_renumbering.json");
    let json_file_str_renumbering = read_to_string(json_file_path_renumbering).expect("file not found");
/*
    let json_file_path_words_renum = Path::new("data/renumbering/words_renumbering.json");
    let json_file_str_words_renum = read_to_string(json_file_path_words_renum).expect("file not found");
*/


    let start = Instant::now();
    // use instead of from_reader
    let sentences:SentencesAsIndicesDynamicsN = serde_json
        ::from_str(&json_file_str_sentences)
        .expect("error while reading json with sentences");
//    println!("The sentences in initial form: {:?}", sentences);
    let renumbering:GeneratedTokensCounting = serde_json
        ::from_str(&json_file_str_renumbering)
        .expect("error while reading json with renumbering");
/*
    let words_renum:GeneratedWordsCounting = serde_json
        ::from_str(&json_file_str_words_renum)
        .expect("error while reading json with renumbering");
*/

    let mut pairs_tokens_matrix_eng:Array1<f32> = Array::zeros(NUMBER_PAIRS*NUMBER_TOKENS_ENG);
    let mut p_t_matrix_eng:Array2<f32> = Array::zeros((NUMBER_PAIRS,NUMBER_TOKENS_ENG));

    let mut pairs_tokens_matrix_fra:Array1<f32> = Array::zeros(NUMBER_PAIRS*NUMBER_TOKENS_FRA);
    let mut p_t_matrix_fra:Array2<f32> = Array::zeros((NUMBER_PAIRS,NUMBER_TOKENS_FRA));

// data to the matrix:  vector of indices like [2,5,7,8,7] is changed to the 1D array 
// where on the 3 -d place is 1, on the 8 -th place is 2, (there are two 7 in the vector), 
// on the 9 -th place is 1 etc....
    for (ixs,collection) in sentences.eng_sentence_flattened_to_token_indices.iter() {
//                println!("The number of keys: {}", &x.sentence_flattened_to_token_indices.keys().len());
// use new indices from renumbering
        let mut new_ind; 
        for ind in collection.iter() {
            new_ind = renumbering.eng_renum_old_new.get(ind).unwrap();
            pairs_tokens_matrix_eng[[ixs*(NUMBER_TOKENS_ENG)+new_ind]] += 1.0;
        }
    }
    p_t_matrix_eng = pairs_tokens_matrix_eng.into_shape((NUMBER_PAIRS,NUMBER_TOKENS_ENG)).unwrap();

    for (ixs,collection) in sentences.fra_sentence_flattened_to_token_indices.iter() {
// use new indices for tokens from renumbering tokens_renumbering
        let mut new_ind; 
        for ind in collection.iter() {
            new_ind = renumbering.fra_renum_old_new.get(ind).unwrap();
            pairs_tokens_matrix_fra[[ixs*(NUMBER_TOKENS_FRA)+new_ind]] += 1.0;
        }
    }
    p_t_matrix_fra = pairs_tokens_matrix_fra.into_shape((NUMBER_PAIRS,NUMBER_TOKENS_FRA)).unwrap();

    println!("Array eng! :{:?}", &p_t_matrix_eng.slice(s![0,..]));
    println!("Shapes eng:\nshape: {:?}\ndim: {:?}\nraw_dim: {:?}"
             ,&p_t_matrix_eng.shape(), &p_t_matrix_eng.dim(), &p_t_matrix_eng.raw_dim());

    println!("Array fra! :{:?}", &p_t_matrix_fra.slice(s![0,..]));
    println!("Shapes fra:\nshape: {:?}\ndim: {:?}\nraw_dim: {:?}"
             ,&p_t_matrix_fra.shape(), &p_t_matrix_fra.dim(), &p_t_matrix_fra.raw_dim());
/*
    let start = Instant::now();
    println!("Start svd calculation");
    let res = p_t_matrix_eng.svd(true, true).unwrap();
    println!("Finish svd calculation");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
*/


let sentences_max_len = SentencesMaxLengths::from_sentences(&sentences);
// array which keeps index of word for every sentence in translation pairs and every ordered 
// position in target sentence, so index (i,j) of the arrays is i-> index of sentence in
// translation pairs; j -> position of word in target sentence
    let mut array_target_words_in_order = Array::from_elem((NUMBER_PAIRS
                                                            ,sentences_max_len.target_sentence_max_len), NOWORD);
// i correnpond to target sentence
    for i in 0..NUMBER_PAIRS {
// k is position index in a target sentence
        for k in 0..sentences.fra_words_as_indices.get(&i).unwrap().len() {
// save at position (i,k) new word index 
            array_target_words_in_order[[i,k]]=sentences.fra_words_as_indices
                .get(&i)
                .unwrap()[k]
                .to_owned();
        }
    }

    println!("array shape: {:?}",&array_target_words_in_order.shape());
    println!("last row of array: {:?}", &array_target_words_in_order.slice(s![185582,..]));

// the set of translation pairs is split on vectors of indices , every vector correspond to the 
// same first word in target sentence (we will predict the first words in the job)
// words of target sentences to collection of sentence indices
    let mut words_to_sentences = WordsInTargetToSentences::new();
    words_to_sentences.from_array_of_words(&array_target_words_in_order);
    words_to_sentences.max_and_min();
    println!("First word to sentences {:?}\n", &words_to_sentences.words_sentences_collections[0]);

// calculate collections of sentences lengths corresponding to words 
// the collections will be features to predict the words 
    let words_to_lengths = WordsToSentenceLengths::from_words_to_sentences(&words_to_sentences,&sentences);
    println!("First words and length collections {:?}\n",&words_to_lengths.words_to_lengths[0]);

    ::serde_json::to_writer(&File::create("data/matrices_generator/words_to_sentences.json")?
                            ,&words_to_sentences)?;
    ::serde_json::to_writer(&File::create("data/matrices_generator/words_to_lengths.json")?
                            ,&words_to_lengths)?;
    ::serde_json::to_writer(&File::create("data/matrices_generator/sentences_max_length.json")?
                            ,&sentences_max_len)?;
// we keep a number > 0 (lengths which is always > 0 here) in number -1 index
// index 0 correspond to length 1
// (pos_t,ixx_t,len)
// the array: how many sentences of length N in source language correspond to the target word ixx_t
// at position in target sentence position pos_t
// index for special 'NoWord' item in target sentence is NOWORD 
/*
    let mut target_words_presence = Array::from_elem((sentences_max_len.target_sentence_max_len
                                                            ,NUMBER_WORDS_FRA
                                                            ,sentences_max_len.source_sentence_max_len), 0);
*/            
    let mut target_words_presence = Array3::<usize>::zeros((sentences_max_len.target_sentence_max_len
                                                            ,NUMBER_WORDS_FRA
                                                            ,sentences_max_len.source_sentence_max_len));

        let mut t =0;
        for map in &words_to_lengths.words_to_lengths {
            for (w,collection) in &map.words {
                for len in collection {
                    target_words_presence[[t,*w,len-1]]+=1;
                }
            }
            t+=1;
        }


    for i in 0..sentences_max_len.target_sentence_max_len {
        println!("Vector of features -> target word in first position:\n{:?}\n"
                 ,&target_words_presence.slice(s![0,i,..]));
    }
    #[derive(Serialize, Deserialize)]
    pub struct TargetWordsPresence {
        words_to_lengths:Array3<usize>,
    }

    impl TargetWordsPresence {
        pub fn from_array(array:&Array3<usize>)-> TargetWordsPresence {
            TargetWordsPresence {
                words_to_lengths:array.to_owned(),
            }
        }
    }

    let lengths_predictors = TargetWordsPresence::from_array(&target_words_presence);

    ::serde_json::to_writer(&File::create("data/lengths_to_predictors/lengths_to_predictors.json")?
                            ,&lengths_predictors)?;

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

