use translationlib::*;

fn main() {
    let mut txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
// eliminate u{202f} non breaking space and '\xa0' which in unicode is u{00a0}
    txt = txt.replace_some_chars("\u{202f}\u{00a0}", ' ');
    txt = txt.replace_some_chars("\"",' ');
    txt.separate_punctuation("?!.,");

    let sentences = SentencesForTranslation::from_corpus(&txt);

    println!("The first part of the text {:?}\n", &txt.processed[0..1000]);
    println!("The first pairs for translation {:?}\n", &sentences.eng[0..10]);
    for i in 0..1000 {
        println!("{:?}         {:?}", sentences.eng[i], sentences.fra[i]); 
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

    println!("The eng words indexation: {:?}\n", &vocab.eng_word_index);
    println!("The eng index to words:  {:?}\n", &vocab.eng_index_word);

    let mut tokens = VocabOfTokens::new();
    tokens.from_word_vocab(&vocab);
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
    tokens.token_to_index_c();
    tokens.index_to_token();
    println!("Initial eng_tokens as indices:\n{:?}",&tokens.eng_token_index);
    println!("Initial eng_index_token representation;\n{:?}",&tokens.eng_index_token);
    
    let mut collection = WordToIndexCollection::new();
    collection.from_word_vocab(&vocab,&tokens);

    println!("IndexToWordsCollection.eng_words_n:\n{:?}",&collection.eng_words_n);
    println!("IndexToWordsCollection.eng_words_s:\n{:?}",&collection.eng_words_s);

    let mut tokens_words_dynamic = TokensAndWordsDynamicsLang::new(Lang::Eng);
    tokens_words_dynamic=TokensAndWordsDynamicsLang::initial_set_from_vocab(Lang::Eng,&tokens,&vocab);
     
    let num_merges = 10;
    let mut condidate_pairs_for_merge; 
    let mut most_frequent_pair;
    for merge in 0..num_merges {
        println!("Iteration: {:?}",merge);
        println!("Before candidate");
        condidate_pairs_for_merge = CandidatesForMergeLang::from_word_vocab(&vocab,&collection,Lang::Eng);
        println!("Before most frequent");
        most_frequent_pair=MostFrequentPairLang::most_frequent_pair(&condidate_pairs_for_merge);
        println!("Before tokens_words_dynamic");
        tokens_words_dynamic.from_most_frequent_pair(&most_frequent_pair);
    }
}
