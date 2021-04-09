use crate::data::ALPHABET;

pub struct Reflector {
    setting: &'static str
}

impl Reflector {
    pub fn a() -> Reflector {
        Reflector::new(REFLECTOR_A)
    }
    pub fn b() -> Reflector {
        Reflector::new(REFLECTOR_B)
    }
    pub fn c() -> Reflector {
        Reflector::new(REFLECTOR_C)
    }

    fn new(setting: &'static str) -> Reflector {
        if setting.len() != ALPHABET.len() {
            panic!(
                "Reflector alphabet must be of same length and contain same characters as '{}'",
                ALPHABET
            );
        }
        Reflector {
            setting
        }
    }

    pub fn encode(&self, i: u8) -> u8 {
        let c = self.setting.chars().nth(i as usize).unwrap();
        let idx = ALPHABET.find(c).unwrap();
        println!("   --- next_encoded: {}", c);
        idx as u8
    }
}

// ---- names: Heer, Enigma A, Heeres, Wehrmacht, Service Enigma, Army/GAF machine
const REFLECTOR_A: &str     = "EJMZALYXVBWFCRQUONTSPIKHGD";
const REFLECTOR_B: &str     = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
const REFLECTOR_C: &str     = "FVPJIAOYEDRZXWGCTKUQSBNMHL";
// ----
