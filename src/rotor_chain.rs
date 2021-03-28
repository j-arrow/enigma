use crate::rotor::Rotor;

struct RotorChain<'a> {
    right: &'a Rotor,
    middle: &'a Rotor,
    left: &'a Rotor
}

impl<'a> RotorChain<'a> {
    pub fn new(right: &'a Rotor, middle: &'a Rotor, left: &'a Rotor) -> RotorChain {
        RotorChain {
            right,
            middle,
            left
        }
    }

    pub fn rotate(&mut self) {
        let did_right_reset = self.right.rotate();
        if did_right_reset {
            let did_middle_reset = self.middle.rotate();
            if did_middle_reset {
                self.left.rotate();
            }
        }
    }
}
