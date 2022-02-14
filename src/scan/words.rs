use crate::frequency_index::FrequencyIndex;
use indicatif::ProgressBar;
use types::jotoba::words::Word;

use super::check_sentence_part;

pub fn run(f_index: &mut FrequencyIndex, words: &[Word]) {
    let bar = ProgressBar::new(words.len() as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:100.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    for word in words {
        scan_word(f_index, word);
        bar.inc(1);
    }
}

fn scan_word(f_index: &mut FrequencyIndex, word: &Word) {
    let furi = match word.get_furigana() {
        Some(f) => f,
        None => return,
    };

    for part in furi {
        check_sentence_part(f_index, &part);
    }
}
