use crate::rotors::rotor::Rotor;
use crate::data::ALPHABET;
use log::{debug};

pub struct RotorChain {
    left: Rotor,
    middle: Rotor,
    right: Rotor
}

impl RotorChain {
    pub fn new(left: Rotor, middle: Rotor, right: Rotor) -> RotorChain {
        RotorChain {
            left,
            middle,
            right
        }
    }

    pub fn encode_from_right(&self, encoded: u8) -> u8 {
        let mut e = self.right.encode_from_right(encoded);
        e = self.middle.encode_from_right(e);
        e = self.left.encode_from_right(e);
        e
    }

    pub fn encode_from_left(&self, encoded: u8) -> u8 {
        let mut e = self.left.encode_from_left(encoded);
        e = self.middle.encode_from_left(e);
        e = self.right.encode_from_left(e);
        e
    }

    pub fn rotate(&mut self) {
        let will_rotate_middle = self.right.rotate();
        if will_rotate_middle || self.middle.is_in_turnover_position() {

            debug!("Will force next rotor rotation due to double step? {}", self.middle.is_in_turnover_position());
            let will_rotate_left = self.middle.rotate();
            if will_rotate_left {
                self.left.rotate();
            }
        }
        debug!("{}", self.get_offsets_string());
    }

    fn get_offsets_string(&self) -> String {
        format!(
            "{}{}{}",
            self.left.get_offset_character(),
            self.middle.get_offset_character(),
            self.right.get_offset_character()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_sequence() {
        let mut r1 = Rotor::enigma_i_wehrmacht_i();
        r1.turn_to_character('A');
        let mut r2 = Rotor::enigma_i_wehrmacht_ii();
        r2.turn_to_character('A');
        let mut r3 = Rotor::enigma_i_wehrmacht_iii();
        r3.turn_to_character('A');

        let mut chain = RotorChain::new(r1, r2, r3);

        assert_eq!("AAA", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("AAB", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("AAC", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("AAD", chain.get_offsets_string());
    }

    #[test]
    fn test_double_step_sequence() {
        let mut r1 = Rotor::enigma_i_wehrmacht_i();
        r1.turn_to_character('A');
        let mut r2 = Rotor::enigma_i_wehrmacht_ii();
        r2.turn_to_character('D');
        let mut r3 = Rotor::enigma_i_wehrmacht_iii();
        r3.turn_to_character('U');

        let mut chain = RotorChain::new(r1, r2, r3);

        assert_eq!("ADU", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("ADV", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("AEW", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("BFX", chain.get_offsets_string());
        chain.rotate();
        assert_eq!("BFY", chain.get_offsets_string());
    }
}
