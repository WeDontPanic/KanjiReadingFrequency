use std::{
    collections::HashMap,
    io::{Read, Write},
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[cfg(feature = "all")]
use types::jotoba::kanji::Kanji;

#[derive(Serialize, Deserialize)]
pub struct FrequencyIndex {
    data: HashMap<char, FreqData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreqData {
    total: usize,
    readings: Vec<(String, u32)>,
}

impl FreqData {
    fn new(readings: Vec<String>) -> Self {
        let readings = readings.into_iter().map(|i| (i, 0)).collect_vec();
        Self { total: 0, readings }
    }

    #[inline]
    pub fn get_total(&self) -> usize {
        self.total
    }

    pub fn get_reading<S: AsRef<str>>(&self, r: S) -> Option<u32> {
        self.readings
            .iter()
            .find(|i| i.0 == r.as_ref())
            .map(|i| i.1)
    }
}

impl FrequencyIndex {
    #[cfg(feature = "all")]
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

    /// Returns a FreqData for the kanji `c`
    pub fn get(&self, c: char) -> Option<&FreqData> {
        self.data.get(&c)
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

    pub fn save<W: Write>(&self, out: W) -> Result<(), bincode::Error> {
        bincode::serialize_into(out, &self)
    }

    pub fn load<R: Read>(read: R) -> Result<Self, bincode::Error> {
        bincode::deserialize_from(read)
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
