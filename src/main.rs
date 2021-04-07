use translationlib::*;

fn main() {
    let mut txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
// eliminate u{202f} non breaking space and '\xa0' which in unicode is u{00a0}
    txt = txt.replace_some_chars("\u{202f}\u{00a0}", ' ');
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
    
}
