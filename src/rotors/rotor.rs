use log::debug;
use crate::enigma::SUPPORTED_ALPHABET;

#[derive(Debug, PartialEq)]
pub struct Rotor {
    current_offset: u8,
    alphabet: &'static str,
    turnover_offsets: Vec<u8>
}

impl Rotor {
    pub fn enigma_i_wehrmacht_i() -> Rotor {
        Rotor::new(ENIGMA_I_WEHRMACHT_I_ROTOR, ENIGMA_I_WEHRMACHT_I_TURNOVER)
    }
    pub fn enigma_i_wehrmacht_ii() -> Rotor {
        Rotor::new(ENIGMA_I_WEHRMACHT_II_ROTOR, ENIGMA_I_WEHRMACHT_II_TURNOVER)
    }
    pub fn enigma_i_wehrmacht_iii() -> Rotor {
        Rotor::new(ENIGMA_I_WEHRMACHT_III_ROTOR, ENIGMA_I_WEHRMACHT_III_TURNOVER)
    }
    pub fn m3_wehrmacht_iv() -> Rotor {
        Rotor::new(M3_WEHRMACHT_IV_ROTOR, M3_WEHRMACHT_IV_TURNOVER)
    }
    pub fn m3_wehrmacht_v() -> Rotor {
        Rotor::new(M3_WEHRMACHT_V_ROTOR, M3_WEHRMACHT_V_TURNOVER)
    }

    fn new(alphabet: &'static str, turnover: &'static str) -> Rotor {
        for c in alphabet.chars() {
            if !SUPPORTED_ALPHABET.contains(c) {
                panic!("Alphabet error: '{}' must be a letter from set '{}'", c, SUPPORTED_ALPHABET);
            }
        }

        let mut turnover_offsets: Vec<u8> = Vec::with_capacity(turnover.chars().count());
        for c in turnover.chars() {
            let i = SUPPORTED_ALPHABET.find(c).expect(
                &format!("Turnover error: '{}' must be a letter from set '{}'", c, SUPPORTED_ALPHABET)
            ) as u8;
            turnover_offsets.push(i);
        }

        Rotor {
            current_offset: 0,
            alphabet,
            turnover_offsets
        }
    }

    pub(crate) fn encode_from_right(&self, i: u8) -> u8 {
        let offseted_i = Rotor::offset_positively(i, self.current_offset);
        let next_encoded = self.alphabet.chars().nth(offseted_i as usize).unwrap();
        debug!("   --- rotor_r: {}", next_encoded);
        let next_i = SUPPORTED_ALPHABET.find(next_encoded).unwrap();
        let next_i = Rotor::offset_negatively(next_i as u8, self.current_offset);
        next_i
    }

    pub(crate) fn encode_from_left(&self, i: u8) -> u8 {
        let offseted_i = Rotor::offset_positively(i, self.current_offset);
        let next_encoded = SUPPORTED_ALPHABET.chars().nth(offseted_i as usize).unwrap();
        let next_i = self.alphabet.find(next_encoded).unwrap();
        debug!("   --- rotor_l: {}", SUPPORTED_ALPHABET.chars().nth(next_i).unwrap());
        let next_i = Rotor::offset_negatively(next_i as u8, self.current_offset);
        next_i
    }

    pub(crate) fn turn_to_character(&mut self, character: char) {
        self.current_offset = match SUPPORTED_ALPHABET.find(character) {
            None => panic!("Character '{}' is not in supported alphabet: {}", character, SUPPORTED_ALPHABET),
            Some(position) => position
        } as u8;
    }

    pub(crate) fn rotate(&mut self) -> bool {
        let should_rotate_next = self.turnover_offsets.contains(&self.current_offset);
        self.current_offset = Rotor::offset_positively(self.current_offset, 1);
        debug!(
            "Rotor steps from '{}' to '{}'. Will rotate next rotor? {}",
            // character BEFORE rotation
            SUPPORTED_ALPHABET.chars().nth(Rotor::offset_negatively(self.current_offset, 1) as usize).unwrap(),
            //character AFTER rotation
            SUPPORTED_ALPHABET.chars().nth(self.current_offset as usize).unwrap(),
            should_rotate_next);
        should_rotate_next
    }

    pub(crate) fn is_in_turnover_position(&self) -> bool {
        self.turnover_offsets.contains(&self.current_offset)
    }

    #[allow(dead_code)] // used in tests
    fn offset_by(&mut self, offset: i8) {
        self.current_offset = if offset.is_positive() {
            Rotor::offset_positively(self.current_offset, offset as u8)
        } else {
            Rotor::offset_negatively(self.current_offset, (offset * -1) as u8)
        };
    }

    #[allow(dead_code)] // used in tests
    pub(in crate::rotors) fn get_offset_character(&self) -> char {
        SUPPORTED_ALPHABET.chars().nth(self.current_offset as usize).unwrap()
    }

    fn offset_positively(offset_source: u8, offset_by: u8) -> u8 {
        // this function is complicated just because I had a goal in life
        // to make operations on offset never go out of range 0..25
        let o = offset_by % 26;
        if o == 0 {
            return offset_source;
        }
        let allowed_max_offset_before_limit = 25 - offset_source;
        let will_offset_over_limit = o > allowed_max_offset_before_limit;
        if will_offset_over_limit {
            if offset_source == 25 {
                o - 1
            } else {
                o - allowed_max_offset_before_limit - 1
            }
        } else { offset_source + o }
    }

    fn offset_negatively(offset_source: u8, offset_by: u8) -> u8 {
        // this function is complicated just because I had a goal in life
        // to make operations on offset never go out of range 0..25
        let o = offset_by % 26;
        if o == 0 {
            offset_source
        } else if offset_source >= o {
            offset_source - o
        } else {
            26 - (o - offset_source)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation() {
        let mut r = Rotor::new(SUPPORTED_ALPHABET, "Q");
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), true);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
        assert_eq!(r.rotate(), false);
    }

    #[test]
    fn rotor_will_cause_turnover_on_expected_points() {
        let mut r = Rotor::new(SUPPORTED_ALPHABET, "DGIKW");

        for _ in 1..=5 {
            assert_eq!(r.rotate(), false);  // A
            assert_eq!(r.rotate(), false);  // B
            assert_eq!(r.rotate(), false);  // C
            assert_eq!(r.rotate(), true);   // -> D
            assert_eq!(r.rotate(), false);  // E
            assert_eq!(r.rotate(), false);  // F
            assert_eq!(r.rotate(), true);   // -> G
            assert_eq!(r.rotate(), false);  // H
            assert_eq!(r.rotate(), true);   // -> I
            assert_eq!(r.rotate(), false);  // J
            assert_eq!(r.rotate(), true);   // -> K
            assert_eq!(r.rotate(), false);  // L
            assert_eq!(r.rotate(), false);  // M
            assert_eq!(r.rotate(), false);  // N
            assert_eq!(r.rotate(), false);  // O
            assert_eq!(r.rotate(), false);  // P
            assert_eq!(r.rotate(), false);  // Q
            assert_eq!(r.rotate(), false);  // R
            assert_eq!(r.rotate(), false);  // S
            assert_eq!(r.rotate(), false);  // T
            assert_eq!(r.rotate(), false);  // U
            assert_eq!(r.rotate(), false);  // V
            assert_eq!(r.rotate(), true);   // -> W
            assert_eq!(r.rotate(), false);  // X
            assert_eq!(r.rotate(), false);  // Y
            assert_eq!(r.rotate(), false);  // Z
        }
    }

    #[test]
    fn positive_offset_works_correctly() {
        assert_eq!(Rotor::offset_positively(0, 10), 10);
        assert_eq!(Rotor::offset_positively(25, 1), 0);
        assert_eq!(Rotor::offset_positively(5, 10), 15);
        assert_eq!(Rotor::offset_positively(10, 10), 20);
        assert_eq!(Rotor::offset_positively(15, 10), 25);
        assert_eq!(Rotor::offset_positively(20, 10), 4);
        assert_eq!(Rotor::offset_positively(10, 100), 6);
        assert_eq!(Rotor::offset_positively(3, 78), 3);
    }

    #[test]
    fn negative_offset_works_correctly() {
        assert_eq!(Rotor::offset_negatively(25, 1), 24);
        assert_eq!(Rotor::offset_negatively(25, 5), 20);
        assert_eq!(Rotor::offset_negatively(25, 10), 15);
        assert_eq!(Rotor::offset_negatively(25, 15), 10);
        assert_eq!(Rotor::offset_negatively(25, 20), 5);
        assert_eq!(Rotor::offset_negatively(25, 25), 0);
        assert_eq!(Rotor::offset_negatively(25, 30), 21);
        assert_eq!(Rotor::offset_negatively(10, 100), 14);
        assert_eq!(Rotor::offset_negatively(3, 78), 3);
    }

    mod turn_to_character {
        use super::*;

        #[test]
        #[should_panic(expected = "Character 'a' is not in supported alphabet")]
        fn test_1() {
            let mut r = Rotor::enigma_i_wehrmacht_i();
            r.turn_to_character('a');
        }

        #[test]
        fn test_2() {
            let mut r = Rotor::new("ABCDEF", "E");
            assert_eq!(r.rotate(), false);
            r.turn_to_character('E');
            assert_eq!(r.rotate(), true);
        }
    }

    mod offset_by {
        use super::*;

        #[test]
        fn test_1() {
            let mut r = Rotor::enigma_i_wehrmacht_i();
            assert_eq!(r.current_offset, 0);
            r.offset_by(10);
            assert_eq!(r.current_offset, 10);
        }

        #[test]
        fn test_2() {
            let mut r = Rotor::enigma_i_wehrmacht_i();
            assert_eq!(r.current_offset, 0);
            r.offset_by(26);
            assert_eq!(r.current_offset, 0);
        }

        #[test]
        fn test_3() {
            let mut r = Rotor::enigma_i_wehrmacht_i();
            assert_eq!(r.current_offset, 0);
            r.offset_by(30);
            assert_eq!(r.current_offset, 4);
        }

        #[test]
        fn test_4() {
            let mut r = Rotor::enigma_i_wehrmacht_i();
            assert_eq!(r.current_offset, 0);
            r.offset_by(-6);
            assert_eq!(r.current_offset, 20);
        }
    }
}

// ---- names: Heer, Enigma A, Heeres, Wehrmacht, Service Enigma, Army/GAF machine
#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_I_ROTOR: &str = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_I_TURNOVER: &str = "Q";

#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_II_ROTOR: &str = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_II_TURNOVER: &str = "E";

#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_III_ROTOR: &str = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
#[allow(dead_code)]
const ENIGMA_I_WEHRMACHT_III_TURNOVER: &str = "V";

#[allow(dead_code)]
const M3_WEHRMACHT_IV_ROTOR: &str = "ESOVPZJAYQUIRHXLNFTGKDCMWB";
#[allow(dead_code)]
const M3_WEHRMACHT_IV_TURNOVER: &str = "J";

#[allow(dead_code)]
const M3_WEHRMACHT_V_ROTOR: &str = "VZBRGITYUPSDNHLXAWMJQOFECK";
#[allow(dead_code)]
const M3_WEHRMACHT_V_TURNOVER: &str = "Z";
// ----
