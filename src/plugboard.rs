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
        if let None = ALPHABET.find(from) {
            panic!("Character '{}' is not in supported alphabet: {}", from, ALPHABET);
        }
        if let None = ALPHABET.find(to) {
            panic!("Character '{}' is not in supported alphabet: {}", to, ALPHABET);
        }

        if from.eq(&to) {
            self.disconnect(from);
            return;
        }

        let disconnected_1 = self.mapping.insert(from, to);
        if let Some(c) = disconnected_1 {
            if !c.eq(&from) && !c.eq(&to) {
                *self.mapping.get_mut(&c).unwrap() = c;
            }
        }

        let disconnected_2 = self.mapping.insert(to, from);
        if !disconnected_1.eq(&disconnected_2) {
            if let Some(c) = disconnected_2 {
                if !c.eq(&from) && !c.eq(&to) {
                    *self.mapping.get_mut(&c).unwrap() = c;
                }
            }
        }
    }

    pub fn disconnect(&mut self, char_to_disconnect: char) {
        if let None = ALPHABET.find(char_to_disconnect) {
            panic!("Character '{}' is not in supported alphabet: {}", char_to_disconnect, ALPHABET);
        }
        let disconnected_value = self.mapping.insert(char_to_disconnect, char_to_disconnect);

        if let Some(c) = disconnected_value {
            if c.eq(&char_to_disconnect) {
                return;
            }
            *self.mapping.get_mut(&c).unwrap() = c;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugboard() {
        let mut plugboard = Plugboard::identity();
        let mut expected_mapping = plugboard.mapping.clone();

        plugboard.connect('A', 'B');
        expected_mapping.insert('A', 'B');
        expected_mapping.insert('B', 'A');
        assert_eq!(expected_mapping, plugboard.mapping);

        plugboard.connect('C', 'D');
        expected_mapping.insert('C', 'D');
        expected_mapping.insert('D', 'C');
        assert_eq!(expected_mapping, plugboard.mapping);

        plugboard.connect('B', 'C');
        expected_mapping.insert('A', 'A');
        expected_mapping.insert('B', 'C');
        expected_mapping.insert('C', 'B');
        expected_mapping.insert('D', 'D');
        assert_eq!(expected_mapping, plugboard.mapping);

        plugboard.disconnect('B');
        expected_mapping.insert('B', 'B');
        expected_mapping.insert('C', 'C');
        assert_eq!(expected_mapping, plugboard.mapping);
    }
}
