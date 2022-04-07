pub mod frequency_index;

#[cfg(feature = "all")]
pub mod joto_resources;

#[cfg(feature = "all")]
pub mod scan;

use std::fs::File;

#[cfg(feature = "all")]
use frequency_index::FrequencyIndex;
#[cfg(feature = "all")]
use japanese::JapaneseExt;
#[cfg(feature = "all")]
use joto_resources::{get_dict_resources, get_sentences};

const JOTO_STORAGE_PATH: &str = "../jotoba/resources/storage_data";
const JOTO_SENTENCE_PATH: &str = "../jotoba/resources/sentences.bin";

#[cfg(feature = "all")]
fn main() {
    let dict_resources = get_dict_resources(JOTO_STORAGE_PATH);
    let sentences = get_sentences(JOTO_SENTENCE_PATH);

    let mut freq_index = FrequencyIndex::new(&dict_resources.kanji);

    println!("Scan sentences");
    scan::sentences::run(&mut freq_index, &sentences);

    println!("Scan words");
    scan::words::run(&mut freq_index, &dict_resources.words);

    println!("Saving");
    freq_index.save(File::create("out").unwrap()).unwrap();
}

#[cfg(feature = "all")]
pub fn readings_match(reading: &str, word: &str) -> bool {
    let reading = reading.replace('-', "");

    if reading.contains('.') {
        let lit_reading = reading.split('.').next().unwrap();
        return lit_reading.to_hiragana() == word.to_hiragana();
    }

    reading.to_hiragana() == word.to_hiragana()
}
