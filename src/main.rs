use crate::rotors::rotor::Rotor;
use crate::rotors::rotor_chain::RotorChain;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::entry_disk::EntryDisk;
use crate::enigma::Enigma;
use simple_logger::SimpleLogger;
use log::{info};

mod data;
mod rotors;
mod reflector;
mod plugboard;
mod entry_disk;
mod enigma;

fn main() {
    SimpleLogger::new().init().unwrap();

    let entry_disk = EntryDisk::identity();

    let plugboard = Plugboard::identity();
    // plugboard.connect('A', 'B');
    // plugboard.connect('C', 'D');
    // plugboard.connect('E', 'F');
    // plugboard.connect('G', 'H');
    // plugboard.connect('I', 'J');

    let mut r1 = Rotor::enigma_i_wehrmacht_i();
    r1.offset_by(2);
    let mut r2 = Rotor::enigma_i_wehrmacht_ii();
    r2.offset_by(1);
    let r3 = Rotor::enigma_i_wehrmacht_iii();
    let rotor_chain = RotorChain::new(r1, r2, r3);

    let reflector = Reflector::b();

    let mut enigma = Enigma::new(entry_disk, plugboard, rotor_chain, reflector);

    let msg = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    info!("====== Plaintext:    {}", msg);
    let encoded = enigma.encode(msg);
    info!("====== Encoded:    {}", encoded);

    // "QSMCXSNGTQSPWGGOQDJHVRRIELKTIGQQKOMBOYOUVGDHTCOEEWKNHHDCOVQZBVBBFSPQQO"
    // "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
}
