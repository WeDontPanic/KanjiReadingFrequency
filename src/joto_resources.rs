use std::{fs::File, io::BufReader};

use resources::models::{storage::SentenceStorage, DictResources};

pub fn get_dict_resources() -> DictResources {
    DictResources::read(BufReader::new(
        File::open("../jotoba/resources/storage_data").unwrap(),
    ))
    .unwrap()
}

pub fn get_sentences() -> SentenceStorage {
    bincode::deserialize_from(BufReader::new(
        File::open("../jotoba/resources/sentences.bin").unwrap(),
    ))
    .unwrap()
}
