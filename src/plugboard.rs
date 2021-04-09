use std::collections::BTreeMap;
use crate::data::ALPHABET;

pub struct Plugboard {
    mapping: BTreeMap<char, char>
}

impl Plugboard {
    pub fn identity() -> Plugboard {
        let mapping = ALPHABET.chars()
            .fold(
                BTreeMap::new(),
                |mut acc, c| {
                    acc.insert(c, c);
                    acc
                }
            );
        Plugboard {
            mapping
        }
    }

    pub fn connect(&mut self, from: char, to: char) {
        panic!("// TODO NOT IMPLEMENTED");
    }

    pub fn encode_from_right(&self, letter: char) -> u8 {
        // TODO fix this method
        for (i, entry) in self.mapping.iter().enumerate() {
            if letter == *entry.0 {
                return i as u8;
            }
        }
        panic!("Plugboard does not support '{}' character", letter);
    }

    pub fn encode_from_left(&self, i: u8) -> char {
        for (idx, entry) in self.mapping.iter().enumerate() {
            if idx == i as usize {
                return *entry.1;
            }
        }
        panic!("Plugboard does not support '{}' index", i);
    }
}
