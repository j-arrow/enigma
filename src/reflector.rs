use log::debug;
use crate::enigma::SUPPORTED_ALPHABET;

#[derive(Debug, PartialEq)]
pub struct Reflector {
    setting: &'static str
}

impl Reflector {
    #[allow(dead_code)]
    pub fn a() -> Reflector {
        Reflector::new(REFLECTOR_A)
    }
    #[allow(dead_code)]
    pub fn b() -> Reflector {
        Reflector::new(REFLECTOR_B)
    }
    #[allow(dead_code)]
    pub fn c() -> Reflector {
        Reflector::new(REFLECTOR_C)
    }

    fn new(setting: &'static str) -> Reflector {
        if setting.len() != SUPPORTED_ALPHABET.len() {
            panic!(
                "Reflector alphabet must be of same length and contain same characters as '{}'",
                SUPPORTED_ALPHABET
            );
        }
        Reflector {
            setting
        }
    }

    pub(crate) fn encode(&self, i: u8) -> u8 {
        let c = self.setting.chars().nth(i as usize).unwrap();
        let idx = SUPPORTED_ALPHABET.find(c).unwrap();
        debug!("   --- reflector: {}", c);
        idx as u8
    }
}

// ---- names: Heer, Enigma A, Heeres, Wehrmacht, Service Enigma, Army/GAF machine
#[allow(dead_code)]
const REFLECTOR_A: &str     = "EJMZALYXVBWFCRQUONTSPIKHGD";
#[allow(dead_code)]
const REFLECTOR_B: &str     = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
#[allow(dead_code)]
const REFLECTOR_C: &str     = "FVPJIAOYEDRZXWGCTKUQSBNMHL";
// ----
