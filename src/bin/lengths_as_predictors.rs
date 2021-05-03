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
    let json_file_path_lengths = Path::new("data/matrices_generator/words_to_lengths.json");
    let json_file_str_lengths = read_to_string(json_file_path_lengths)
        .expect("file ..words_to_lengths..not found");
    let json_file_path_words_vocab = Path::new("data/matrices_generator/vocab.json");
    let json_file_str_words_vocab = read_to_string(json_file_path_words_vocab)
        .expect("file ..vocab..not found");

/*
    let json_file_path_words_renum = Path::new("data/renumbering/words_renumbering.json");
    let json_file_str_words_renum = read_to_string(json_file_path_words_renum)
        .expect("file ..words_renum..not found");
*/
    let json_file_path_max_length = Path::new("data/matrices_generator/sentences_max_length.json");
    let json_file_str_max_length = read_to_string(json_file_path_max_length)
        .expect("file ..max_length.. not found");


    let start = Instant::now();
    // use instead of from_reader
    let words_as_lengths_collections:WordsToSentenceLengths = serde_json
        ::from_str(&json_file_str_lengths)
        .expect("error while reading json with lengths");

    let words_vocab:Vocab = serde_json
        ::from_str(&json_file_str_words_vocab)
        .expect("error while reading json with words vocab");

//    println!("The sentences in initial form: {:?}", sentences);
/*
    let words_renum:GeneratedWordsCounting = serde_json
        ::from_str(&json_file_str_words_renum)
        .expect("error while reading json with words renumbering");
*/
    let sentences_max_length:SentencesMaxLengths = serde_json
        ::from_str(&json_file_str_max_length)
        .expect("error while reading json with sentences max lenght");

    println!("Sentences max lengths fra: {}   and   eng: {}\n"
             , sentences_max_length.target_sentence_max_len
             ,sentences_max_length.source_sentence_max_len);
    println!("Number of eng: {}  and  fra {} words"
             ,words_vocab.eng_words_total
             ,words_vocab.fra_words_total);

// array (translation_pair,position_of_word_in target_sentence,lengths_of_sentences_in_source) 
// of dimention:
// NUMBER_PAIRS*sentences_max_length.target_sentence_max_len*sentences_max_length.source_sentence_max_len
// we will keep number of sentences with length 'len' in last index of the array which is 'len-1'

    let mut lengths_features = Array::from_elem((NUMBER_PAIRS
                                                      ,sentences_max_length.target_sentence_max_len
                                                      ,sentences_max_length.source_sentence_max_len),0);
    for position in 0..words_as_lengths_collections.words_to_lengths.len() {
        for (ixs,source_collection) in &words_as_lengths_collections.words_to_lengths[position].words {
            for pos  in 0..source_collection.len() {
//                println!("source_collection[pos] {:?}", &source_collection[pos]);
                lengths_features[[*ixs,position,usize::from(source_collection[pos])-1]]+=1;
            }
            
        }
    }
    
    println!("Array of features last line {:?}", lengths_features.slice(s![185582,..,..]));
/*
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
//                    println!("ixs: {}, ind: {}", ixs, ind);
//                    println!("flat array index: {}, pairs_tokens_matrix elt: {}\n",
//                             ixs*(NUMBER_TOKENS) + ind, pairs_tokens_matrix[[ixs*(NUMBER_TOKENS)+ind]]);
        }
    }
    p_t_matrix_eng = pairs_tokens_matrix_eng.into_shape((NUMBER_PAIRS,NUMBER_TOKENS_ENG)).unwrap();

    for (ixs,collection) in sentences.fra_sentence_flattened_to_token_indices.iter() {
//                println!("The number of keys: {}", &x.sentence_flattened_to_token_indices.keys().len());
// use new indices from renumbering
        let mut new_ind; 
        for ind in collection.iter() {
            new_ind = renumbering.fra_renum_old_new.get(ind).unwrap();
            pairs_tokens_matrix_fra[[ixs*(NUMBER_TOKENS_FRA)+new_ind]] += 1.0;
//                    println!("ixs: {}, ind: {}", ixs, ind);
//                    println!("flat array index: {}, pairs_tokens_matrix elt: {}\n",
//                             ixs*(NUMBER_TOKENS) + ind, pairs_tokens_matrix[[ixs*(NUMBER_TOKENS)+ind]]);
        }
    }
    p_t_matrix_fra = pairs_tokens_matrix_fra.into_shape((NUMBER_PAIRS,NUMBER_TOKENS_FRA)).unwrap();

    println!("Array eng! :{:?}", &p_t_matrix_eng.slice(s![0,..]));
    println!("Shapes eng:\nshape: {:?}\ndim: {:?}\nraw_dim: {:?}"
             ,&p_t_matrix_eng.shape(), &p_t_matrix_eng.dim(), &p_t_matrix_eng.raw_dim());

    println!("Array fra! :{:?}", &p_t_matrix_fra.slice(s![0,..]));
    println!("Shapes fra:\nshape: {:?}\ndim: {:?}\nraw_dim: {:?}"
             ,&p_t_matrix_fra.shape(), &p_t_matrix_fra.dim(), &p_t_matrix_fra.raw_dim());

//TODO move the max length calc to struct
//max lengh of sentences (target) in tokens
    let mut target_sentence_max_len = 0;
    for (ixs, vec) in &sentences.fra_words_as_indices {
        if vec.len() >= target_sentence_max_len {
            target_sentence_max_len=vec.len();
        }
    }
// array which keeps for every target sentence indices of sentence words , if length of a sentence 
//is smaller than max, usize::MAX is added instead of a word index
    let mut array_target_words_in_order = Array::from_elem((NUMBER_PAIRS,target_sentence_max_len), Exist::Abs);
    for i in 0..NUMBER_PAIRS {
        for k in 0..sentences.fra_words_as_indices.get(&i).unwrap().len() {
            array_target_words_in_order[[i,k]]=Exist::Ex(
                sentences.fra_words_as_indices
                .get(&i)
                .unwrap()[k]
                .to_owned()
                );
        }
    }

    println!("array shape: {:?}",&array_target_words_in_order.shape());
    println!("last row of array: {:?}", &array_target_words_in_order.slice(s![185582,..]));

// words of target sentences to collection of sentence indices
    let mut words_to_sentences = WordsInTargetToSentences::new();
    words_to_sentences.from_array_of_words(&array_target_words_in_order);
    words_to_sentences.max_and_min();
    println!("First word to sentences {:?}\n", &words_to_sentences.words_sentences_collections[0]);

// calculate collections of sentences lengths corresponding to words 
// the collections will be features to predict the words 
    let mut words_to_lengths = WordsToSentenceLengths::from_words_to_sentences(&words_to_sentences,&sentences);
    println!("First words and length collections {:?}\n",&words_to_lengths.words_to_lengths[0]);

    ::serde_json::to_writer(&File::create("data/words_in_order_to_sentences.json")?, &words_to_sentences)?;
*/
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    Ok(())
}

