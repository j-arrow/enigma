use crate::enigma::SUPPORTED_ALPHABET;

pub struct EntryDisk {
    alphabet: &'static str,
}

impl EntryDisk {
    pub fn identity() -> EntryDisk {
        EntryDisk::new(SUPPORTED_ALPHABET)
    }

    fn new(alphabet: &'static str) -> EntryDisk {
        // TODO panic if alphabets dont contain all same characters
        EntryDisk { alphabet }
    }

    pub(crate) fn encode_from_right(&self, i: u8) -> u8 {
        let character = SUPPORTED_ALPHABET
            .chars()
            .nth(i as usize)
            .expect(&format!("Entry disk contains no character at index {}", i));
        self.alphabet.find(character).unwrap() as u8
    }

    pub(crate) fn encode_from_left(&self, i: u8) -> u8 {
        let character = self
            .alphabet
            .chars()
            .nth(i as usize)
            .expect(&format!("Entry disk contains no character at index {}", i));
        SUPPORTED_ALPHABET.find(character).unwrap() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode_from_right {
        use super::*;

        #[test]
        fn test_first_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_right(0);
            assert_eq!(encoded, 0);
        }

        #[test]
        fn test_middle_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_right(13);
            assert_eq!(encoded, 13);
        }

        #[test]
        fn test_last_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_right(25);
            assert_eq!(encoded, 25);
        }

        #[test]
        #[should_panic(expected = "Entry disk contains no character at index 26")]
        fn test_illegal_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_right(26);
            assert_eq!(encoded, 26);
        }
    }

    mod encode_from_left {
        use super::*;

        #[test]
        fn test_first_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_left(0);
            assert_eq!(encoded, 0);
        }

        #[test]
        fn test_middle_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_left(13);
            assert_eq!(encoded, 13);
        }

        #[test]
        fn test_last_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_left(25);
            assert_eq!(encoded, 25);
        }

        #[test]
        #[should_panic(expected = "Entry disk contains no character at index 26")]
        fn test_illegal_index() {
            let disk = EntryDisk::identity();
            let encoded = disk.encode_from_left(26);
            assert_eq!(encoded, 26);
        }
    }
}
