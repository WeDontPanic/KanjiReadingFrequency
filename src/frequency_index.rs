use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use types::jotoba::kanji::Kanji;

#[derive(Serialize, Deserialize)]
pub struct FrequencyIndex {
    data: HashMap<char, FreqData>,
}

#[derive(Serialize, Deserialize)]
struct FreqData {
    total: usize,
    readings: Vec<(String, u32)>,
}

impl FreqData {
    fn new(readings: Vec<String>) -> Self {
        let readings = readings.into_iter().map(|i| (i, 0)).collect_vec();
        Self { total: 0, readings }
    }
}

impl FrequencyIndex {
    pub fn new(all_kanji: &[Kanji]) -> FrequencyIndex {
        let mut data = HashMap::new();

        for kanji in all_kanji {
            let mut readings = vec![];

            let on = kanji.onyomi.clone().unwrap_or_default();
            let kun = kanji.kunyomi.clone().unwrap_or_default();

            for reading in on.into_iter().chain(kun.into_iter()) {
                readings.push(reading);
            }

            data.insert(kanji.literal, FreqData::new(readings));
        }

        FrequencyIndex { data }
    }

    pub fn add_reading<F>(&mut self, kanji_lit: char, matches: F) -> bool
    where
        F: Fn(&str) -> bool,
    {
        let entry = match self.data.get_mut(&kanji_lit) {
            Some(s) => s,
            None => return false,
        };

        let mut c = 0;

        // Count all readings up
        for mr in entry.readings.iter_mut().filter(|i| matches(&i.0)) {
            mr.1 += 1;
            c += 1;
        }

        let success = c > 0;

        if success {
            // We're passing one reading. If there are multiple entries for one single entry,
            // they're treated equally, so we're counting up all matches but only counting one
            // total
            entry.total += 1;
        }

        success
    }

    pub fn debug(&self) {
        for (k, v) in self.data.iter() {
            if v.total == 0 {
                continue;
            }

            let readings = v
                .readings
                .iter()
                .filter(|i| i.1 > 0)
                .map(|i| format!("({}: {})", i.0, i.1))
                .join(", ");

            println!("{k}: {readings} => {}", v.total);
        }
    }
}
