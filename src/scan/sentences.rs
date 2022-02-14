use indicatif::ProgressBar;
use resources::models::storage::SentenceStorage;

use crate::frequency_index::FrequencyIndex;

use super::check_sentence_part;

pub fn run(f_index: &mut FrequencyIndex, sentences: &SentenceStorage) {
    let bar = ProgressBar::new(sentences.sentences.len() as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:100.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    for sentence in sentences.sentences.iter() {
        let jp_txt = &sentence.1.japanese;
        let furi_txt = &sentence.1.furigana;

        scan_sentence(f_index, (jp_txt, furi_txt));
        bar.inc(1);
    }
}

fn scan_sentence(f_index: &mut FrequencyIndex, (_jp_txt, furi_txt): (&str, &str)) {
    let furi_iter = japanese::furigana::from_str(&furi_txt);

    for i in furi_iter {
        check_sentence_part(f_index, &i);
    }
}
