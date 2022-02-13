pub mod frequency_index;
pub mod joto_resources;
pub mod scan;

use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use frequency_index::FrequencyIndex;
use joto_resources::{get_dict_resources, get_sentences};
use once_cell::sync::Lazy;

/// The path of the unidict-mecab dictionary
pub const NL_PARSER_PATH: &str = "./unidic-mecab";

/// A global natural language parser
pub static JA_NL_PARSER: Lazy<igo_unidic::Parser> =
    Lazy::new(|| igo_unidic::Parser::new(NL_PARSER_PATH).unwrap());

fn main() {
    let dict_resources = get_dict_resources();
    let sentences = get_sentences();

    let freq_index = FrequencyIndex::new(&dict_resources.kanji);
    let freq_index_m = Arc::new(Mutex::new(freq_index));

    let start = Instant::now();
    println!("Scan sentences");
    scan::sentences::run(freq_index_m.clone(), &sentences);
    println!("Sentence scanning took: {:?}", start.elapsed());

    println!("Scan words");
    scan::words::run(freq_index_m.clone(), &dict_resources.words);

    //freq_index_m.lock().unwrap().debug();
}

pub fn readings_match(reading: &str, word: &str) -> bool {
    let reading = reading.replace('-', "");

    if reading.contains('.') {
        let lit_reading = reading.split('.').next().unwrap();
        return lit_reading == word;
    }

    reading == word
}
