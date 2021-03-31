use translationlib::*;

fn main() {
    let mut txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
// eliminate u{202f} non breaking space and '\xa0' which in unicode is u{00a0}
    txt = txt.replace_some_chars("\u{202f}\u{00a0}", ' ');
    txt.separate_punctuation("?!.,");

    let sentences = SentencesForTranslation::from_corpus(&txt);

//    println!("The first part of the text {:?}\n", &txt.processed[0..1000]);
//    println!("The first pairs for translation {:?}\n", &sentences.eng[0..10]);
    for i in 0..1000 {
        println!("{:?}         {:?}", sentences.eng[i], sentences.fra[i]); 
    }

    let translation_pairs = TranslationPairs::from_sentences(&sentences);

    println!("{:?}", &translation_pairs.pairs[0..200]);

    let mut vocab = Vocab::new();
    vocab.vector_words(&sentences);
    println!("The words: \n{:?}",&vocab.eng_set[0..50]);
}

