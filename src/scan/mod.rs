use japanese::furigana::SentencePartRef;

use crate::{frequency_index::FrequencyIndex, readings_match};

pub mod sentences;
pub mod words;

pub fn check_sentence_part(f_index: &mut FrequencyIndex, i: &SentencePartRef) {
    if i.kanji.is_none() {
        return;
    }
    let kanji = i.kanji.unwrap();

    if kanji.chars().count() > 1 {
        return;
    }

    let kanji_lit = kanji.chars().next().unwrap();

    f_index.add_reading(kanji_lit, |r| readings_match(r, i.kana));
}
