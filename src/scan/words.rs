use crate::frequency_index::FrequencyIndex;
use indicatif::ProgressBar;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use types::jotoba::words::Word;

pub fn run(f_index: Arc<Mutex<FrequencyIndex>>, words: &[Word]) {
    let bar = ProgressBar::new(words.len() as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:100.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    words.par_iter().for_each(|word| {
        //
        bar.inc(1);
    });
}
