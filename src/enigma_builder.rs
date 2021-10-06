use crate::entry_disk::EntryDisk;
use crate::reflector::Reflector;
use crate::enigma::Enigma;
use crate::plugboard::{Plugboard, PlugboardConnection};
use crate::rotors::rotor::Rotor;
use crate::rotors::rotor_chain::RotorChain;

pub struct EnigmaBuilder {
    entry_disk: Option<EntryDisk>,
    reflector: Option<Reflector>,
    rotor_left: Option<Rotor>,
    rotor_middle: Option<Rotor>,
    rotor_right: Option<Rotor>,
    plugboard_connections: Vec<PlugboardConnection>,
}

impl EnigmaBuilder {
    pub fn init() -> Self {
        EnigmaBuilder {
            entry_disk: Some(EntryDisk::identity()),
            reflector: None,
            rotor_left: None,
            rotor_middle: None,
            rotor_right: None,
            plugboard_connections: vec![],
        }
    }

    pub fn entry_disk(&mut self, entry_disk: EntryDisk) -> &mut Self {
        self.entry_disk = Some(entry_disk);
        self
    }

    pub fn reflector(mut self, reflector: Reflector) -> Self {
        self.reflector = Some(reflector);
        self
    }

    pub fn rotor_left(mut self, rotor_left: Rotor) -> Self {
        self.rotor_left = Some(rotor_left);
        self
    }

    pub fn rotor_middle(mut self, rotor_middle: Rotor) -> Self {
        self.rotor_middle = Some(rotor_middle);
        self
    }

    pub fn rotor_right(mut self, rotor_right: Rotor) -> Self {
        self.rotor_right = Some(rotor_right);
        self
    }

    pub fn plugboard_connections(mut self, plugboard_connections: Vec<PlugboardConnection>) -> Self {
        self.plugboard_connections = plugboard_connections;
        self
    }

    pub fn build(&mut self) -> Enigma {
        let mut plugboard = Plugboard::identity();
        for pc in &self.plugboard_connections {
            plugboard.connect(pc.left, pc.right);
        }

        let rotor_chain = RotorChain::new(
            self.rotor_left.take().expect("Left rotor is required"),
            self.rotor_middle.take().expect("Middle rotor is required"),
            self.rotor_right.take().expect("Right rotor is required")
        );
        Enigma::new(
            plugboard,
            self.entry_disk.take().expect("Entry disk is required"),
            rotor_chain,
            self.reflector.take().expect("Reflector is required")
        )
    }
}
