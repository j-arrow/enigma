use std::collections::BTreeMap;
use crate::data::ALPHABET;

pub struct EntryDisk {
    mapping: BTreeMap<char, char>
}

impl EntryDisk {
    pub fn identity() -> EntryDisk {
        EntryDisk::new(ALPHABET)
    }

    fn new(setting: &str) -> EntryDisk {
        if setting.len() != ALPHABET.len() {
            panic!(
                "Entry disk alphabet must be of same length and contain same characters as '{}'",
                ALPHABET
            );
        }
        let mut mapping: BTreeMap<char, char> = BTreeMap::new();
        for (i, c) in ALPHABET.chars().enumerate() {
            mapping.insert(c, setting.chars().nth(i).unwrap());
        }
        EntryDisk {
            mapping
        }
    }

    pub fn encode(&self, i: u8) -> u8 {
        // TODO
        i
    }
}
