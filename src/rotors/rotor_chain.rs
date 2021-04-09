use crate::rotors::rotor::Rotor;
use crate::data::ALPHABET;

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
        let did_right_reset = self.right.rotate();
        if did_right_reset {
            let did_middle_reset = self.middle.rotate();
            if did_middle_reset {
                self.left.rotate();
            }
        }
        println!(
            "Offset: {} {} {}",
            self.left.get_offset(),
            self.middle.get_offset(),
            self.right.get_offset()
        );
    }
}
