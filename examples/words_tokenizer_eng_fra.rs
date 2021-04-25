use translationlib::*;
//use serde_json::{Result, Value};

//#[macro_use] extern crate serde_derive;
use std::fs::File;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let mut txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
/*
// eliminate u{202f} non breaking space and '\xa0' which in unicode is u{00a0}
    txt = txt.replace_some_chars("\u{202f}\u{00a0}", ' ');
*/
    txt = txt.replace_some_chars("\"",' ');
    txt.separate_punctuation("?!.,");

    let mut sentences = SentencesForTranslation::from_corpus(&txt);
    sentences.from_sentence();

    println!("The first part of the text {:?}\n", &txt.processed[0..1000]);
    println!("The number of pairs for translation: {:?}\n", sentences.size);
    println!("The first pairs for translation:\n");
    for i in 0..1000 {
        println!("First sentences {:?}         {:?}"
                 ,sentences.eng.get(&i).unwrap()
                 ,sentences.fra.get(&i).unwrap()); 
    }

    let translation_pairs = TranslationPairs::from_sentences(&sentences);

    println!("First translation pairs{:?}", &translation_pairs.pairs[0..200]);

    let mut vocab = Vocab::new();
    vocab.list_of_words(&sentences);
    println!("The firsts eng words: \n{:?}",&vocab.eng_set[0..50]);
    println!("The firsts fra words: \n{:?}",&vocab.fra_set[0..50]);

    vocab.words_and_quantity();
    println!("The eng words and quantity: \n{:?}", &vocab.eng_words);
    println!("The fra words and quantity: \n{:?}", &vocab.fra_words);


    println!("The number of eng words: {:?}", &vocab.eng_words_total);
    println!("The number of fra words: {:?}", &vocab.fra_words_total);
    
    vocab.word_to_index();
    vocab.index_to_word();
    vocab.index_quantity();

//////    println!("The eng words indexation: {:?}\n", &vocab.eng_word_index);
//////    println!("The eng index to words:  {:?}\n", &vocab.eng_index_word);

    let mut tokens = VocabOfTokens::new();
    tokens.from_word_vocab(&vocab);
    tokens.token_to_index_c();
    tokens.index_to_token();
    tokens.index_to_quantity();
//    tokens.quantity_of_tokens();
    println!("Eng tokens ! {:?}", &tokens.eng_token_quantity);      
    println!("Fra tokens ! {:?}", &tokens.fra_token_quantity);      

    println!("Number of initial tokens eng: {}",
             &tokens.eng_token_total);
    println!("Number of initial tokens fra: {}",
             &tokens.fra_token_total);
// convert the initial tokens (which are (character as string)s ) to the number representation , 
// we use Ind type for the numbers 
//    tokens.token_to_index_c();
//    tokens.index_to_token();
    println!("Initial eng_tokens as indices:\n{:?}",&tokens.eng_token_index);
    println!("Initial eng_index_token representation;\n{:?}",&tokens.eng_index_token);

    println!("Initial fra_tokens as indices:\n{:?}",&tokens.fra_token_index);
    println!("Initial fra_index_token representation;\n{:?}",&tokens.fra_index_token);

    
    let mut collection = WordToIndexCollection::new();
    collection.from_word_vocab(&vocab,&tokens);

    println!("IndexToWordsCollection.eng_words_n:\n{:?}",&collection.eng_words_n);
//    println!("IndexToWordsCollection.eng_words_s:\n{:?}",&collection.eng_words_s);
    println!("IndexToWordsCollection.fra_words_n:\n{:?}",&collection.fra_words_n);

//
    let mut sentences_indices = SentencesAsIndices::new();
    sentences_indices.from_word_vocab(&vocab,&sentences);
    sentences_indices.from_word_as_tokens(&collection);


    let mut tokens_words_dynamic=TokensAndWordsDynamicsN
        ::initial_set_from_vocab(&tokens,&vocab);
    let mut words_as_substrings = TokensAndWordsDynamicsN
        ::word_as_strings_collection(&tokens_words_dynamic);
     
    let num_merges = 2000;
    let mut condidate_pairs_for_merge; 
    let mut most_frequent_pair;
    let mut entropy; 
    for merge in 0..num_merges {
        println!("Iteration: {:?}",merge);
        condidate_pairs_for_merge = CandidatesForMergeN::from_tokens_words_dynamic(&tokens_words_dynamic);
        most_frequent_pair=MostFrequentPairN::most_frequent_pair(&condidate_pairs_for_merge);
        println!("Most frequent pair eng: {:?}", most_frequent_pair.eng_pair);
        println!("Most frequent pair fra: {:?}", most_frequent_pair.fra_pair);
        
        tokens_words_dynamic.from_most_frequent_pair(&most_frequent_pair);
        words_as_substrings = TokensAndWordsDynamicsN
            ::word_as_strings_collection(&tokens_words_dynamic);

// calculate entropy 
        entropy = tokens_words_dynamic.tokens_vocab_and_entropy();

        println!(" Eng entropy:   {:?} ; Tokens number: {:?}", entropy.eng_entropy, entropy.eng_dyn_tokens.keys().len());
        println!(" Fra entropy:   {:?} ; Tokens number: {:?}", entropy.fra_entropy, entropy.fra_dyn_tokens.keys().len());


        println!(" Eng word indices:\n{:?}"
                 ,&tokens_words_dynamic.eng_word_indices.get(&17206).unwrap());
        println!(" Fra word indices:\n{:?}"
                 ,&tokens_words_dynamic.fra_word_indices.get(&17206).unwrap());


        println!(" Eng word substrings:\n{:?}"
                 ,&words_as_substrings.eng_word_tokens.get(&17206).unwrap());
        println!(" Fra word substrings:\n{:?}"
                 ,&words_as_substrings.fra_word_tokens.get(&17206).unwrap());

    }
/*
    match tokens_words_dynamic {
        TokensAndWordsDynamicsLang
            ::Eng(ref x) => println!(" Eng word indices:\n{:?}",&x.word_indices.get(&17206).unwrap()),
        _=> println!("Somethin is wrong with word_indices printing"),

    }
*/


    let mut sentence_as_indices_dynamics = SentencesAsIndicesDynamicsN
        ::initial_from_sentences_and_indices(&sentences_indices);
    sentence_as_indices_dynamics.from_tokens_words_dynamic(&tokens_words_dynamic);

    println!("Eng sentences as words(in token indices):\n{:?}"
                                     ,&sentence_as_indices_dynamics.eng_words_as_token_indices
                                     .get(&17206).unwrap());
    println!("Fra sentences as words(in token indices):\n{:?}"
             ,&sentence_as_indices_dynamics.fra_words_as_token_indices
             .get(&17206).unwrap());

    println!("Eng sentences as flattened as tokens:\n{:?}"
                                     ,&sentence_as_indices_dynamics.eng_sentence_flattened_to_token_indices
                                     .get(&17206).unwrap());

    println!("Fra sentences as flattened as tokens:\n{:?}"
                                     ,&sentence_as_indices_dynamics.fra_sentence_flattened_to_token_indices
                                     .get(&17206).unwrap()); 

// serialize and deserialize
/*
    let serialized = serde_json::to_string(&sentence_as_indices_dynamics).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: SentencesAsIndicesDynamicsLang = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
*/

    ::serde_json::to_writer(&File::create("data/data.json")?, &sentence_as_indices_dynamics)?;

    Ok(())
}
