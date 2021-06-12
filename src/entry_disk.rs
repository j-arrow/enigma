use crate::data::ALPHABET;

pub struct EntryDisk {
    alphabet: &'static str
}

impl EntryDisk {
    pub fn identity() -> EntryDisk {
        EntryDisk::new(ALPHABET)
    }

    fn new(alphabet: &'static str) -> EntryDisk {
        EntryDisk {
            alphabet
        }
    }

    pub fn encode_from_right(&self, i: u8) -> u8 {
        let character = ALPHABET.chars().nth(i as usize).unwrap();
        self.alphabet.find(character).unwrap() as u8
    }

    pub fn encode_from_left(&self, i: u8) -> u8 {
        let character = self.alphabet.chars().nth(i as usize).unwrap();
        ALPHABET.find(character).unwrap() as u8
    }
}
