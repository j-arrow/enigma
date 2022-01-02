use std::fmt::Display;

use crate::enigma::Enigma;
use crate::entry_disk::EntryDisk;
use crate::plugboard::{Plugboard, PlugboardConnection};
use crate::reflector::Reflector;
use crate::rotors::rotor::Rotor;
use crate::rotors::rotor_chain::RotorChain;

#[derive(Debug)]
pub enum RotorPlacement {
	Left,
	Middle,
	Right
}
impl Display for RotorPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RotorPlacement::Left => f.write_str("Left"),
            RotorPlacement::Middle => f.write_str("Middle"),
            RotorPlacement::Right => f.write_str("Right"),
        }
    }
}


#[derive(Debug)]
pub enum BuildError {
	RotorError(RotorPlacement, String),
	PlugboardError(String),
	EntryDiskError(String),
	ReflectorError(String)
}

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

    pub fn entry_disk(mut self, entry_disk: EntryDisk) -> Self {
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

    pub fn plugboard_connections(
        mut self,
        plugboard_connections: Vec<PlugboardConnection>,
    ) -> Self {
        self.plugboard_connections = plugboard_connections;
        self
    }

    pub fn build(&mut self) -> Result<Enigma, BuildError> {
		if let Err(e) = self.validate_ready_to_build() {
			return Err(e);
		}

        let mut plugboard = Plugboard::identity();
        for pc in &self.plugboard_connections {
            if let Err(e) = plugboard.connect(pc.left, pc.right) {
				return Err(BuildError::PlugboardError(e));
			}
        }

        let rotor_chain = RotorChain::new(
        	self.rotor_left.take().unwrap(),
            self.rotor_middle.take().unwrap(),
            self.rotor_right.take().unwrap()
        );
        Ok(
			Enigma::new(
				plugboard,
				self.entry_disk.take().unwrap(),
				rotor_chain,
				self.reflector.take().unwrap()
        	)
		)
    }

	fn validate_ready_to_build(&self) -> Result<(), BuildError> {
		if let None = self.rotor_left {
			return Err(BuildError::RotorError(RotorPlacement::Left, "Left rotor is required".into()));
		}
		if let None = self.rotor_middle {
			return Err(BuildError::RotorError(RotorPlacement::Middle, "Middle rotor is required".into()));
		}
		if let None = self.rotor_right {
			return Err(BuildError::RotorError(RotorPlacement::Right, "Right rotor is required".into()));
		}
		if let None = self.entry_disk {
			return Err(BuildError::EntryDiskError("Entry disk is required".into()));
		}
		if let None = self.reflector {
			return Err(BuildError::ReflectorError("Reflector is required".into()));
		}
		Ok(())
	}
}
