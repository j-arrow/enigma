use crate::rotors::rotor::Rotor;
use log::{debug};
use crate::enigma::SUPPORTED_ALPHABET;

pub struct RotorChain {
    left: Rotor,
    middle: Rotor,
    right: Rotor
}

impl RotorChain {
    pub(crate) fn new(left: Rotor, middle: Rotor, right: Rotor) -> RotorChain {
        RotorChain {
            left,
            middle,
            right
        }
    }

    pub(crate) fn change_setting<S: AsRef<str>>(&mut self, new_setting: S) {
        let new_setting_ref = new_setting.as_ref();
        if new_setting_ref.len() != 3 {
            panic!("New setting for rotor chain is unsupported. Required 3 characters, got {}.",
                   new_setting_ref.len());
        }

        for c in new_setting_ref.chars() {
            let option = SUPPORTED_ALPHABET.find(c);
            option.expect(
                &format!("Character '{}' is not in supported alphabet: {}.", c, SUPPORTED_ALPHABET)
            );
        }

        self.left.turn_to_character(new_setting_ref.chars().nth(0).unwrap());
        self.middle.turn_to_character(new_setting_ref.chars().nth(1).unwrap());
        self.right.turn_to_character(new_setting_ref.chars().nth(2).unwrap());
    }

    pub(crate) fn encode_from_right(&self, encoded: u8) -> u8 {
        let mut e = self.right.encode_from_right(encoded);
        e = self.middle.encode_from_right(e);
        e = self.left.encode_from_right(e);
        e
    }

    pub(crate) fn encode_from_left(&self, encoded: u8) -> u8 {
        let mut e = self.left.encode_from_left(encoded);
        e = self.middle.encode_from_left(e);
        e = self.right.encode_from_left(e);
        e
    }

    pub(crate) fn rotate(&mut self) {
        let will_rotate_middle = self.right.rotate();
        if will_rotate_middle || self.middle.is_in_turnover_position() {
            debug!("Will force next rotor rotation due to double step? {}", self.middle.is_in_turnover_position());
            let will_rotate_left = self.middle.rotate();
            if will_rotate_left {
                self.left.rotate();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod change_setting {
        use super::*;

        #[test]
        #[should_panic(expected = "New setting for rotor chain is unsupported. Required 3 characters, got 1.")]
        fn will_panic_on_too_short_new_setting() {
            let r1 = Rotor::enigma_i_wehrmacht_i();
            let r2 = Rotor::enigma_i_wehrmacht_ii();
            let r3 = Rotor::enigma_i_wehrmacht_iii();

            let mut chain = RotorChain::new(r1, r2, r3);
            chain.change_setting("X");
        }

        #[test]
        #[should_panic(expected = "New setting for rotor chain is unsupported. Required 3 characters, got 6.")]
        fn will_panic_on_too_long_new_setting() {
            let r1 = Rotor::enigma_i_wehrmacht_i();
            let r2 = Rotor::enigma_i_wehrmacht_ii();
            let r3 = Rotor::enigma_i_wehrmacht_iii();

            let mut chain = RotorChain::new(r1, r2, r3);
            chain.change_setting("XXXXXX");
        }

        #[test]
        #[should_panic(expected = "Character '1' is not in supported alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ.")]
        fn will_panic_on_unknown_character() {
            let r1 = Rotor::enigma_i_wehrmacht_i();
            let r2 = Rotor::enigma_i_wehrmacht_ii();
            let r3 = Rotor::enigma_i_wehrmacht_iii();

            let mut chain = RotorChain::new(r1, r2, r3);
            chain.change_setting("XX1");
        }

        #[test]
        fn sets_new_setting_for_rotors_in_chain() {
            let mut r1 = Rotor::enigma_i_wehrmacht_i();
            r1.turn_to_character('A');
            let mut r2 = Rotor::enigma_i_wehrmacht_ii();
            r2.turn_to_character('A');
            let mut r3 = Rotor::enigma_i_wehrmacht_iii();
            r3.turn_to_character('A');

            let mut chain = RotorChain::new(r1, r2, r3);

            chain.change_setting("XYZ");

            assert_eq!('X', chain.left.get_offset_character());
            assert_eq!('Y', chain.middle.get_offset_character());
            assert_eq!('Z', chain.right.get_offset_character());
        }
    }

    #[test]
    fn test_normal_sequence() {
        let r1 = Rotor::enigma_i_wehrmacht_i();
        let r2 = Rotor::enigma_i_wehrmacht_ii();
        let r3 = Rotor::enigma_i_wehrmacht_iii();
        let mut chain = RotorChain::new(r1, r2, r3);
        chain.change_setting("AAA");

        assert_eq!("AAA", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("AAB", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("AAC", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("AAD", get_offsets_string_for_chain(&chain));
    }

    #[test]
    fn test_double_step_sequence() {
        let r1 = Rotor::enigma_i_wehrmacht_i();
        let r2 = Rotor::enigma_i_wehrmacht_ii();
        let r3 = Rotor::enigma_i_wehrmacht_iii();

        let mut chain = RotorChain::new(r1, r2, r3);
        chain.change_setting("ADU");

        assert_eq!("ADU", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("ADV", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("AEW", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("BFX", get_offsets_string_for_chain(&chain));
        chain.rotate();
        assert_eq!("BFY", get_offsets_string_for_chain(&chain));
    }

    fn get_offsets_string_for_chain(chain: &RotorChain) -> String {
        return format!(
            "{}{}{}",
            chain.left.get_offset_character(),
            chain.middle.get_offset_character(),
            chain.right.get_offset_character()
        )
    }
}
