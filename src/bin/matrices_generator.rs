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
use ndarray::*;
use ndarray_linalg::*;

static NUMBER_TOKENS:Ind = 650;
static NUMBER_PAIRS:usize = 185583;

//static NUMBER_TOKENS:Ind = 650;
//static NUMBER_PAIRS:usize = 100000;



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
    let sentences:SentencesAsIndicesDynamicsLang = serde_json
        ::from_str(&json_file_str)
        .expect("error while reading json");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("The sentences in initial form: {:?}", sentences);

    let mut pairs_tokens_matrix:Array1<f32> = Array::zeros(NUMBER_PAIRS*NUMBER_TOKENS);
    let mut p_t_matrix:Array2<f32> = Array::zeros((NUMBER_PAIRS,NUMBER_TOKENS));


    match sentences {
        SentencesAsIndicesDynamicsLang::Eng(x)=> {
            for (ixs,collection) in x.sentence_flattened_to_token_indices.iter() {
//                println!("The number of keys: {}", &x.sentence_flattened_to_token_indices.keys().len());
                for ind in collection.iter() {
                    pairs_tokens_matrix[[ixs*(NUMBER_TOKENS)+ind]] += 1.0/(*&collection.len() as f32);
//                    println!("ixs: {}, ind: {}", ixs, ind);
//                    println!("flat array index: {}, pairs_tokens_matrix elt: {}\n",
//                             ixs*(NUMBER_TOKENS) + ind, pairs_tokens_matrix[[ixs*(NUMBER_TOKENS)+ind]]);
                }
            }
            p_t_matrix = pairs_tokens_matrix.into_shape((NUMBER_PAIRS,NUMBER_TOKENS)).unwrap();


        },
            
        _ => (),
    }


    println!("Array ! :{:?}", &p_t_matrix.slice(s![0,..]));
    println!("Shapes:\nshape: {:?}\ndim: {:?}\nraw_dim: {:?}"
             ,&p_t_matrix.shape(), &p_t_matrix.dim(), &p_t_matrix.raw_dim());


    let start = Instant::now();
    println!("Start svd calculation");
    let res = p_t_matrix.svd(true, true).unwrap();
/*
//    let (e, vecs) = p_t_matrix.clone().eig().unwrap();
    let a = arr2(&[[2.0, 1.0, 2.0], [-2.0, 2.0, 1.0], [1.0, 2.0, -2.0]]);
    let (e, vecs) = a.clone().eig().unwrap();
*/
    println!("Finish svd calculation");
    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

/*   for i in 0..572 {
        println!("The last raw of matrix {}",p_t_matrix[[185582,i]]);
    }
*/


}

/*
pub struct SentencesAsIndicesDynamics {
    pub words_as_indices:BTreeMap<Ixs,Vec<Ixx>>,
    pub words_as_token_indices:BTreeMap<Ixs,Vec<Vec<Ind>>>,
    pub sentence_flattened_to_token_indices:BTreeMap<Ixs,Vec<Ind>>
}
*/
