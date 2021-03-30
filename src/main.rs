use translationlib::*;

fn main() {
    let txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
    txt = CorpusAsString::replace_some_chars(&txt);

    println!("The first part of the text {:?}\n", &txt.original[0..1000]);

}
