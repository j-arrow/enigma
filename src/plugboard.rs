use crate::enigma::SUPPORTED_ALPHABET;
use std::collections::BTreeMap;

pub struct Plugboard {
    mapping: BTreeMap<char, char>,
}

impl Plugboard {
    pub(crate) fn identity() -> Plugboard {
        let mapping = SUPPORTED_ALPHABET
            .chars()
            .fold(BTreeMap::new(), |mut acc, c| {
                acc.insert(c, c);
                acc
            });
        Plugboard { mapping }
    }

    pub(crate) fn connect(&mut self, from: char, to: char) -> Result<(), String> {
        if let None = SUPPORTED_ALPHABET.find(from) {
			return Err(format!(
                "Character '{}' is not in supported alphabet: {}",
                from, SUPPORTED_ALPHABET
			));
        }
        if let None = SUPPORTED_ALPHABET.find(to) {
            return Err(format!(
                "Character '{}' is not in supported alphabet: {}",
                to, SUPPORTED_ALPHABET
            ));
        }

        if from.eq(&to) {
            if let Err(e) = self.disconnect(from) {
				return Err(e);
			}
            return Ok(());
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
		Ok(())
    }

    pub(crate) fn disconnect(&mut self, char_to_disconnect: char) -> Result<(), String> {
        if let None = SUPPORTED_ALPHABET.find(char_to_disconnect) {
            return Err(format!(
                "Character '{}' is not in supported alphabet: {}",
                char_to_disconnect, SUPPORTED_ALPHABET
            ));
        }
        let disconnected_value = self.mapping.insert(char_to_disconnect, char_to_disconnect);

        if let Some(c) = disconnected_value {
            if c.eq(&char_to_disconnect) {
                return Ok(());
            }
            *self.mapping.get_mut(&c).unwrap() = c;
        }
		Ok(())
    }

    pub(crate) fn encode_from_right(&self, letter: char) -> u8 {
        let encoded = self.mapping.get(&letter).expect(&format!(
            "Plugboard does not support '{}' character",
            letter
        ));
        SUPPORTED_ALPHABET.find(*encoded).unwrap() as u8
    }

    pub(crate) fn encode_from_left(&self, i: u8) -> char {
        let encoded = SUPPORTED_ALPHABET.chars().nth(i as usize).unwrap();
        *self.mapping.get(&encoded).unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct PlugboardConnection {
    pub(crate) left: char,
    pub(crate) right: char,
}

impl PlugboardConnection {
    pub fn create(character_pair: &str) -> Result<PlugboardConnection, String> {
		if character_pair.chars().count() != 2 {
            return Err(format!(
                "Expected only pairs (2 values) split by comma character (,), but found: {}",
                character_pair
            ));
        }

        let p0 = character_pair.chars().nth(0).unwrap();
        let p1 = character_pair.chars().nth(1).unwrap();
        if !SUPPORTED_ALPHABET.contains(p0) {
            return Err(format!(
                "Value '{}' is not allowed as part of plugboard. Allowed: {}",
                p0, SUPPORTED_ALPHABET
            ));
        }
        if !SUPPORTED_ALPHABET.contains(p1) {
            return Err(format!(
                "Value '{}' is not allowed as part of plugboard. Allowed: {}",
                p1, SUPPORTED_ALPHABET
            ));
        }

        Ok(PlugboardConnection {
            left: p0,
            right: p1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugboard() {
        let mut plugboard = Plugboard::identity();
        let mut expected_mapping = plugboard.mapping.clone();

        assert_eq!(plugboard.connect('A', 'B'), Ok(()));
        expected_mapping.insert('A', 'B');
        expected_mapping.insert('B', 'A');
        assert_eq!(expected_mapping, plugboard.mapping);

        assert_eq!(plugboard.connect('C', 'D'), Ok(()));
        expected_mapping.insert('C', 'D');
        expected_mapping.insert('D', 'C');
        assert_eq!(expected_mapping, plugboard.mapping);

        assert_eq!(plugboard.connect('B', 'C'), Ok(()));
        expected_mapping.insert('A', 'A');
        expected_mapping.insert('B', 'C');
        expected_mapping.insert('C', 'B');
        expected_mapping.insert('D', 'D');
        assert_eq!(expected_mapping, plugboard.mapping);

        assert_eq!(plugboard.disconnect('B'), Ok(()));
        expected_mapping.insert('B', 'B');
        expected_mapping.insert('C', 'C');
        assert_eq!(expected_mapping, plugboard.mapping);
    }

    #[test]
    fn expect_disconnect_executed_by_connecting_same_character() {
        let mut plugboard = Plugboard::identity();
        let mut expected_mapping = plugboard.mapping.clone();

        assert_eq!(plugboard.connect('A', 'B'), Ok(()));
        expected_mapping.insert('A', 'B');
        expected_mapping.insert('B', 'A');
        assert_eq!(expected_mapping, plugboard.mapping);

        assert_eq!(plugboard.connect('B', 'B'), Ok(()));
        expected_mapping.insert('A', 'A');
        expected_mapping.insert('B', 'B');
        assert_eq!(expected_mapping, plugboard.mapping);
    }

    #[test]
    fn error_on_connect_for_unsupported_from_character() {
        let mut plugboard = Plugboard::identity();
        assert_eq!(
			plugboard.connect('1', 'A'),
			Err("Character '1' is not in supported alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ".into())
		);
    }

    #[test]
    fn error_on_connect_for_unsupported_to_character() {
        let mut plugboard = Plugboard::identity();
        assert_eq!(
			plugboard.connect('A', '2'),
			Err("Character '2' is not in supported alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ".into())
		);
    }

    #[test]
    fn error_on_disconnect_for_unsupported_character() {
        let mut plugboard = Plugboard::identity();
        assert_eq!(
			plugboard.disconnect('3'),
			Err("Character '3' is not in supported alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ".into())
		);
    }
}
