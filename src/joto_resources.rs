use std::{fs::File, io::BufReader, path::Path};

use resources::models::{storage::SentenceStorage, DictResources};

pub fn get_dict_resources<P: AsRef<Path>>(path: P) -> DictResources {
    DictResources::read(BufReader::new(File::open(path).unwrap())).unwrap()
}

pub fn get_sentences<P: AsRef<Path>>(path: P) -> SentenceStorage {
    bincode::deserialize_from(BufReader::new(File::open(path).unwrap())).unwrap()
}
