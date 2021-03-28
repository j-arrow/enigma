pub struct Rotor {
    offset: u8
}

impl Rotor {
    pub fn new(offset: u8) -> Rotor {
        if offset > 26 {
            panic!("Rotor offset must be within 0..26 range");
        }
        Rotor {
            offset
        }
    }

    pub fn rotate(&mut self) -> bool {
        self.offset = if self.offset == 26 { 0 } else { self.offset + 1};
        self.offset == 0
    }
}
