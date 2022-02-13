use std::sync::{Arc, Mutex};

use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};
use resources::models::storage::SentenceStorage;

use crate::{frequency_index::FrequencyIndex, readings_match};

pub fn run(f_index: Arc<Mutex<FrequencyIndex>>, sentences: &SentenceStorage) {
    let bar = ProgressBar::new(sentences.sentences.len() as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:100.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    // @@@@: remove take
    let sentence_iter = sentences.sentences.iter().par_bridge();
    sentence_iter.for_each(|sentence| {
        let jp_txt = &sentence.1.japanese;
        let furi_txt = &sentence.1.furigana;

        scan_sentence(f_index.clone(), (jp_txt, furi_txt));
        bar.inc(1);
    });
}

fn scan_sentence(f_index: Arc<Mutex<FrequencyIndex>>, (_jp_txt, furi_txt): (&str, &str)) {
    let furi_iter = japanese::furigana::from_str(&furi_txt);

    for i in furi_iter {
        if i.kanji.is_none() {
            continue;
        }
        let kanji = i.kanji.unwrap();

        if kanji.chars().count() > 1 {
            continue;
        }

        let kanji_lit = kanji.chars().next().unwrap();

        let mut l = f_index.lock().unwrap();
        l.add_reading(kanji_lit, |r| readings_match(r, i.kana));
    }
}
