// GLOSSARY:
// reflector = UKW
// entry disc = ETW

use crate::entry_disk::EntryDisk;
use crate::plugboard::Plugboard;
use crate::reflector::Reflector;
use crate::rotors::rotor_chain::RotorChain;

pub const SUPPORTED_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct EncodingResult {
    pub message_length: usize,
    pub basic_position: String,
    pub encoded_message_key: String,
    pub encoded_message: String,
}

pub struct Enigma {
    plugboard: Plugboard,
    entry_disk: EntryDisk,
    rotor_chain: RotorChain,
    reflector: Reflector,
}

impl Enigma {
    pub(crate) fn new(
        plugboard: Plugboard,
        entry_disk: EntryDisk,
        rotor_chain: RotorChain,
        reflector: Reflector,
    ) -> Enigma {
        Enigma {
            plugboard,
            entry_disk,
            rotor_chain,
            reflector,
        }
    }

    pub fn encode(
        &mut self,
        basic_position: String,
        message_key: String,
        message: String,
    ) -> Result<EncodingResult, String> {
        let mut message_vector = vec![];
        let mut message_errors = vec![];

        for c in message.chars() {
            if c.is_whitespace() {
                message_vector.push(String::from('X'));
            } else {
                // TODO part converting to uppercase should be moved to encoding method
                let uppercase = c.to_uppercase().to_string();
                if SUPPORTED_ALPHABET.contains(&uppercase) {
                    message_vector.push(uppercase);
                } else {
                    message_errors.push(format!("Unsupported character '{}'", c));
                }
            }
        }

        if !message_errors.is_empty() {
            return Err(message_errors.join(", "));
        }

        // 1. Set rotors to positions of 'basic_position'
        if let Err(e) = self.rotor_chain.change_setting(&basic_position) {
            return Err(e);
        }

        // 2. Encode 'message_key' and read encoded string
        let encoded_message_key = self.encode_for_current_rotor_setting(&message_key);

        // 3. Set rotors to positions of 'message_key'
        if let Err(e) = self.rotor_chain.change_setting(&message_key) {
            return Err(e);
        }

        // 4. Encode the message using 'message_key' rotor setting
        let msg = message_vector.join("");
        let encoded_message = self.encode_for_current_rotor_setting(&msg);

        // 5. Return required values for printing message
        Ok(EncodingResult {
            message_length: message.len(),
            basic_position,
            encoded_message_key,
            encoded_message,
        })
    }

    fn encode_for_current_rotor_setting(&mut self, msg: &String) -> String {
        let mut v: Vec<char> = Vec::with_capacity(msg.chars().count());
        for c in msg.chars() {
            self.rotor_chain.rotate();

            let c_encoded = self.plugboard.encode_from_right(c);
            let c_encoded = self.entry_disk.encode_from_right(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_right(c_encoded);
            let c_encoded = self.reflector.encode(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_left(c_encoded);
            let c_encoded = self.entry_disk.encode_from_left(c_encoded);
            let c_encoded = self.plugboard.encode_from_left(c_encoded);
            v.push(c_encoded);
        }
        v.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rotors::rotor::Rotor;
    use simple_logger::SimpleLogger;
    use std::sync::Once;

    static START: Once = Once::new();

    fn init() {
        START.call_once(|| {
            SimpleLogger::new().init().unwrap();
        });
    }

    #[test]
    fn test_encoding_for_same_character_sequence() {
        init();

        let initial_rotor_settings = "AAA";
        let decoded = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let encoded = "BDZGOWCXLTKSBTMCDLPBMUQOFXYHCXTGYJFLINHNXSHIUNTHEORXPQPKOVHCBUBTZSZSOO";

        test_enigma_i(initial_rotor_settings, decoded, encoded);
        test_enigma_i(initial_rotor_settings, encoded, decoded);
    }

    #[test]
    fn test_encoding_for_alphabet_sequence() {
        init();

        let initial_rotor_settings = "AAA";
        let decoded = "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST";
        let encoded = "BJELRQZVJWARXSNBXORSTJPKHEARBCAFENSBJKCMOXAXQTKEQORPQGYJJKUPZYCXFBGLXZ";

        test_enigma_i(initial_rotor_settings, decoded, encoded);
        test_enigma_i(initial_rotor_settings, encoded, decoded);
    }

    #[test]
    fn test_encoding_for_double_set_sequence_rotations_1() {
        init();

        let initial_rotor_settings = "AAA";
        let decoded = "LOREMIPSUMISSIMPLYDUMMYTEXTOFTHEPRINTINGANDTYPESETTINGINDUSTRYLOREMIPSUMHASBEEN\
            THEINDUSTRYSSTANDARDDUMMYTEXTEVERSINCETHEFIFTEENHUNDREDSWHENANUNKNOWNPRINTERTOOKAGALLEYO\
            FTYPEANDSCRAMBLEDITTOMAKEATYPESPECIMENBOOK";
        let encoded = "PIXWHLIFPVBAJQABBCIXAZGWCAGLSJQYXBWPZCEPXTWMBSNFAGBQJDMYXPXRLLIZH\
            TXQQDVLJOXYBXXXFBYQUCBCBRMCVWCAZDXCCJAXLLSIOZZNICFKSKGLDGVQTOAIQPVHBBZ\
            BVKPPKGTGPYSBBSFBVJVPSBZQWVNJVQJNZWFPTRCZNMCQQIGVXVDGYGGMBJQJLLKSRYGAANGCS";

        test_enigma_i(initial_rotor_settings, decoded, encoded);
        test_enigma_i(initial_rotor_settings, encoded, decoded);
    }

    #[test]
    fn test_encoding_for_double_set_sequence_rotations_2() {
        init();

        let initial_rotor_settings = "GDU";
        let decoded = "LOREMIPSUMISSIMPLYDUMMYTEXTOFTHEPRINTINGANDTYPESETTINGINDUSTRYLOREMIPSUMHASBEEN\
            THEINDUSTRYSSTANDARDDUMMYTEXTEVERSINCETHEFIFTEENHUNDREDSWHENANUNKNOWNPRINTERTOOKAGALLEYO\
            FTYPEANDSCRAMBLEDITTOMAKEATYPESPECIMENBOOK";
        let encoded = "FXJZGYKDITUGTBWEYJWKUAQEFQPIOUPXVSSJDBLMYGKVSXLLRQIYJDGYGZFWZXWGF\
            GUTVEJQEWXDDOCRDGPRWEUCUSQRIICJPTVTKBQUHAZDXTKBARGQQQPBDWTBMDTMIMGPPUI\
            DNWCRLJJTTLZLFBJRSWJBDIDILNMBXEBEUHXUPJHZBZPLXKLGRBCYSEZWMASMPRTKWOJVCHHJO";

        test_enigma_i(initial_rotor_settings, decoded, encoded);
        test_enigma_i(initial_rotor_settings, encoded, decoded);
    }

    mod test_encoding_for_custom_plugboard {
        use super::*;

        #[test]
		fn test_1() {
            let mut plugboard = Plugboard::identity();
            let _ = plugboard.connect('A', 'B');
            let _ = plugboard.connect('E', 'F');
            let _ = plugboard.connect('H', 'I');
            let _ = plugboard.connect('N', 'O');
            let _ = plugboard.connect('T', 'U');
            let _ = plugboard.connect('X', 'Y');

            test_enigma_i_with_custom_plugboard(
                "AAA",
                plugboard,
                "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST",
                "BDFLGOZAUWBRYCNAYNRYTJGEIFFAAJFEFOSPVKCMNLRYVGKFFKUYQGKYJYBPZXCZYAGLYF",
            );
        }

        #[test]
        fn test_2() {
            let mut plugboard = Plugboard::identity();
            let _ = plugboard.connect('A', 'E');
            let _ = plugboard.connect('G', 'L');
            let _ = plugboard.connect('Q', 'X');

            test_enigma_i_with_custom_plugboard(
                "EJO",
                plugboard,
                "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST",
                "SQBZSUPLCQJPOXVIBYKHHFRZGHKTAGVZPWIMVEESBHRLRGQFCGPPRLXJCBTNQMBQKEETFX",
            );
        }
    }

    fn test_enigma_i(initial_rotor_settings: &str, decoded: &str, encoded: &str) {
        let plugboard = Plugboard::identity();
        test_enigma_i_with_custom_plugboard(initial_rotor_settings, plugboard, decoded, encoded);
    }

    fn test_enigma_i_with_custom_plugboard(
        initial_rotor_settings: &str,
        plugboard: Plugboard,
        decoded: &str,
        encoded: &str,
    ) {
        let entry_disk = EntryDisk::identity();

        let mut r1 = Rotor::enigma_i_wehrmacht_i();
        r1.turn_to_character(initial_rotor_settings.chars().nth(0).unwrap());
        let mut r2 = Rotor::enigma_i_wehrmacht_ii();
        r2.turn_to_character(initial_rotor_settings.chars().nth(1).unwrap());
        let mut r3 = Rotor::enigma_i_wehrmacht_iii();
        r3.turn_to_character(initial_rotor_settings.chars().nth(2).unwrap());

        let rotor_chain = RotorChain::new(r1, r2, r3);

        let reflector = Reflector::b();

        let mut enigma = Enigma::new(plugboard, entry_disk, rotor_chain, reflector);

        assert_eq!(
            enigma.encode_for_current_rotor_setting(&String::from(decoded)),
            encoded
        );
    }
}
