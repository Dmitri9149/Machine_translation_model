use translationlib::*;

fn main() {
    let mut txt = CorpusAsString::corpus_from_file("data/fra_eng/fra.txt");
    txt = txt.replace_some_chars(".\n", 'P');

    println!("The first part of the text {:?}\n", &txt.processed[0..1000]);

}
