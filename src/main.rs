use crate::rotor::Rotor;

mod rotor;
mod rotor_chain;
mod reflector;

fn main() {
    let r = Rotor::new(0);
    let r3 = Rotor::new(27);
}
