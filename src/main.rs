use crate::rotors::rotor::Rotor;
use crate::rotors::rotor_chain::RotorChain;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::entry_disk::EntryDisk;

mod data;
mod rotors;
mod reflector;
mod plugboard;
mod entry_disk;

// GLOSSARY:
// reflector = UKW
// entry disc = ETW

struct Enigma {
    entry_disk: EntryDisk,
    plugboard: Plugboard,
    rotor_chain: RotorChain,
    reflector: Reflector
}
impl Enigma {
    fn new(entry_disk: EntryDisk, plugboard: Plugboard, rotor_chain: RotorChain, reflector: Reflector) -> Enigma {
        Enigma {
            entry_disk,
            plugboard,
            rotor_chain,
            reflector
        }
    }

    fn encode(&mut self, msg: &str) -> String {
        let mut v: Vec<char> = Vec::new();
        for c in msg.chars() {
            self.rotor_chain.rotate();

            let c_encoded = self.plugboard.encode_from_right(c);
            let c_encoded = self.entry_disk.encode(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_right(c_encoded);
            let c_encoded = self.reflector.encode(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_left(c_encoded);
            let c_encoded = self.entry_disk.encode(c_encoded);
            let c_encoded = self.plugboard.encode_from_left(c_encoded);

            v.push(c_encoded);

            println!();
        }
        v.iter().collect()
    }
}



fn main() {
    let entry_disk = EntryDisk::identity();

    let mut plugboard = Plugboard::identity();
    // plugboard.connect('A', 'B');
    // plugboard.connect('C', 'D');
    // plugboard.connect('E', 'F');
    // plugboard.connect('G', 'H');
    // plugboard.connect('I', 'J');

    let mut r1 = Rotor::enigma_i_wehrmacht_i();
    r1.offset_by(2);
    let mut r2 = Rotor::enigma_i_wehrmacht_ii();
    r2.offset_by(1);
    let mut r3 = Rotor::enigma_i_wehrmacht_iii();
    let rotor_chain = RotorChain::new(r1, r2, r3);

    let reflector = Reflector::b();

    let mut enigma = Enigma::new(entry_disk, plugboard, rotor_chain, reflector);

    let msg = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    println!("====== Plaintext:    {}", msg);
    let encoded = enigma.encode(msg);
    println!("====== Encoded:    {}", encoded);

    // "QSMCXSNGTQSPWGGOQDJHVRRIELKTIGQQKOMBOYOUVGDHTCOEEWKNHHDCOVQZBVBBFSPQQO"
    // "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
}
