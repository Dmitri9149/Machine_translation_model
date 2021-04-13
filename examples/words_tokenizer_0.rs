use translationlib::*;

fn main() {
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
        println!("{:?}         {:?}"
                 ,sentences.eng.get(&i).unwrap()
                 ,sentences.fra.get(&i).unwrap()); 
    }

    let translation_pairs = TranslationPairs::from_sentences(&sentences);

    println!("{:?}", &translation_pairs.pairs[0..200]);

    let mut vocab = Vocab::new();
    vocab.list_of_words(&sentences);
    println!("The words: \n{:?}",&vocab.eng_set[0..50]);

    vocab.words_and_quantity();
    println!("The hash: \n{:?}", &vocab.eng_words);

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
    println!("Tokens ! {:?}", &tokens.eng_token_quantity);      
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
    
    let mut collection = WordToIndexCollection::new();
    collection.from_word_vocab(&vocab,&tokens);

    println!("IndexToWordsCollection.eng_words_n:\n{:?}",&collection.eng_words_n);
//    println!("IndexToWordsCollection.eng_words_s:\n{:?}",&collection.eng_words_s);
//
    let mut sentences_indices = SentencesAsIndices::new();
    sentences_indices.from_word_vocab(&vocab,&sentences);
    sentences_indices.from_word_as_tokens(&collection);


    let mut tokens_words_dynamic=TokensAndWordsDynamicsLang
        ::initial_set_from_vocab(Lang::Eng,&tokens,&vocab);
    let mut words_as_substrings = TokensAndWordsDynamicsLang
        ::word_as_strings_collection(&tokens_words_dynamic);
     
    let num_merges = 5;
    let mut condidate_pairs_for_merge; 
    let mut most_frequent_pair;
    for merge in 0..num_merges {
        println!("Iteration: {:?}",merge);
        condidate_pairs_for_merge = CandidatesForMergeLang::from_tokens_words_dynamic(&tokens_words_dynamic);
        println!("Before most frequent");
        most_frequent_pair=MostFrequentPairLang::most_frequent_pair(&condidate_pairs_for_merge);
        println!("Before tokens_words_dynamic");
        match most_frequent_pair { 
            MostFrequentPairLang::Eng(ref x) => println!("Most frequent pair eng: {:?}", x.pair),
            _ => println!(" Something is wrong with printing most frequent pair")                                                
        }
        tokens_words_dynamic.from_most_frequent_pair(&most_frequent_pair);
        words_as_substrings = TokensAndWordsDynamicsLang
            ::word_as_strings_collection(&tokens_words_dynamic);


        match tokens_words_dynamic {
            TokensAndWordsDynamicsLang
                ::Eng(ref x) => println!(" Eng word indices:\n{:?}",&x.word_indices.get(&17206).unwrap()),
            _=> println!("Somethin is wrong with word_indices printing"),

        }

        match words_as_substrings {
            WordsAsTokensLang
                ::Eng(ref x) => println!(" Eng word substrings:\n{:?}",&x.word_tokens.get(&17206).unwrap()),
            _=> println!("Somethin is wrong with word_tokens printing"),

        }
    }
/*
    match tokens_words_dynamic {
        TokensAndWordsDynamicsLang
            ::Eng(ref x) => println!(" Eng word indices:\n{:?}",&x.word_indices.get(&17206).unwrap()),
        _=> println!("Somethin is wrong with word_indices printing"),

    }
*/


    let mut sentence_as_indices_dynamics = SentencesAsIndicesDynamicsLang
        ::initial_from_sentences_and_indices(&Lang::Eng,&sentences_indices);
    println!("after let");
    sentence_as_indices_dynamics.from_tokens_words_dynamic(&tokens_words_dynamic);
    println!("after from token...");

    match sentence_as_indices_dynamics {
        SentencesAsIndicesDynamicsLang
            ::Eng(ref x) => println!(" Sentences as tokens:\n{:?}"
//                                     ,&x.sentence_flattened_to_token_indices.get(&17206).unwrap()),
                                     ,&x.words_as_token_indices.get(&17206).unwrap()),

        _=> println!("Somethin is wrong with the sentence printing!"),

    }

}
