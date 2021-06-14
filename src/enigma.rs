// GLOSSARY:
// reflector = UKW
// entry disc = ETW

use crate::entry_disk::EntryDisk;
use crate::plugboard::Plugboard;
use crate::rotors::rotor_chain::RotorChain;
use crate::reflector::Reflector;

pub struct Enigma {
    entry_disk: EntryDisk,
    plugboard: Plugboard,
    rotor_chain: RotorChain,
    reflector: Reflector
}

impl Enigma {
    pub fn new(entry_disk: EntryDisk, plugboard: Plugboard, rotor_chain: RotorChain, reflector: Reflector) -> Enigma {
        Enigma {
            entry_disk,
            plugboard,
            rotor_chain,
            reflector
        }
    }

    pub fn encode(&mut self, msg: &str) -> String {
        let mut v: Vec<char> = Vec::with_capacity(msg.chars().count());
        for (i, c) in msg.chars().enumerate() {
            self.rotor_chain.rotate();

            let c_encoded = self.plugboard.encode_from_right(c);
            let c_encoded = self.entry_disk.encode_from_right(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_right(c_encoded);
            let c_encoded = self.reflector.encode(c_encoded);
            let c_encoded = self.rotor_chain.encode_from_left(c_encoded);
            let c_encoded = self.entry_disk.encode_from_left(c_encoded);
            let c_encoded = self.plugboard.encode_from_left(c_encoded);

            if i != 0 && i % 5 == 0 {
                v.push(' ');
            }
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
        test_enigma_i(
            initial_rotor_settings,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "BDZGO WCXLT KSBTM CDLPB MUQOF XYHCX TGYJF LINHN XSHIU NTHEO RXPQP KOVHC BUBTZ SZSOO"
        );
        test_enigma_i(
            initial_rotor_settings,
            "BDZGOWCXLTKSBTMCDLPBMUQOFXYHCXTGYJFLINHNXSHIUNTHEORXPQPKOVHCBUBTZSZSOO",
            "AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA AAAAA"
        );
    }

    #[test]
    fn test_encoding_for_alphabet_sequence() {
        init();

        let initial_rotor_settings = "AAA";
        test_enigma_i(
            initial_rotor_settings,
            "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST",
            "BJELR QZVJW ARXSN BXORS TJPKH EARBC AFENS BJKCM OXAXQ TKEQO RPQGY JJKUP ZYCXF BGLXZ"
        );
        test_enigma_i(
            initial_rotor_settings,
            "BJELRQZVJWARXSNBXORSTJPKHEARBCAFENSBJKCMOXAXQTKEQORPQGYJJKUPZYCXFBGLXZ",
            "ABCDE FGHIJ KLMNO PQRST UWXYZ ZYXWU TSRQP ONMLK JIHGF EDCBA ABCDE FGHIJ KLMNO PQRST"
        );
    }

    #[test]
    fn test_encoding_for_double_set_sequence_rotations_1() {
        init();

        let initial_rotor_settings = "AAA";
        test_enigma_i(
            initial_rotor_settings,
            "LOREMIPSUMISSIMPLYDUMMYTEXTOFTHEPRINTINGANDTYPESETTINGINDUSTRYLOREMIPSUMHASBEEN\
            THEINDUSTRYSSTANDARDDUMMYTEXTEVERSINCETHEFIFTEENHUNDREDSWHENANUNKNOWNPRINTERTOOKAGALLEYO\
            FTYPEANDSCRAMBLEDITTOMAKEATYPESPECIMENBOOK",
            "PIXWH LIFPV BAJQA BBCIX AZGWC AGLSJ QYXBW PZCEP XTWMB SNFAG BQJDM YXPXR LLIZH \
            TXQQD VLJOX YBXXX FBYQU CBCBR MCVWC AZDXC CJAXL LSIOZ ZNICF KSKGL DGVQT OAIQP VHBBZ \
            BVKPP KGTGP YSBBS FBVJV PSBZQ WVNJV QJNZW FPTRC ZNMCQ QIGVX VDGYG GMBJQ JLLKS RYGAA NGCS"
        );
        test_enigma_i(
            initial_rotor_settings,
            "PIXWHLIFPVBAJQABBCIXAZGWCAGLSJQYXBWPZCEPXTWMBSNFAGBQJDMYXPXRLLIZHTXQQDVLJOXYBXX\
            XFBYQUCBCBRMCVWCAZDXCCJAXLLSIOZZNICFKSKGLDGVQTOAIQPVHBBZBVKPPKGTGPYSBBSFBVJVPSBZQWVNJVQJ\
            NZWFPTRCZNMCQQIGVXVDGYGGMBJQJLLKSRYGAANGCS",
            "LOREM IPSUM ISSIM PLYDU MMYTE XTOFT HEPRI NTING ANDTY PESET TINGI NDUST RYLOR \
            EMIPS UMHAS BEENT HEIND USTRY SSTAN DARDD UMMYT EXTEV ERSIN CETHE FIFTE ENHUN DREDS \
            WHENA NUNKN OWNPR INTER TOOKA GALLE YOFTY PEAND SCRAM BLEDI TTOMA KEATY PESPE CIMEN BOOK"
        );
    }

    #[test]
    fn test_encoding_for_double_set_sequence_rotations_2() {
        init();

        let initial_rotor_settings = "GDU";
        test_enigma_i(
            initial_rotor_settings,
            "LOREMIPSUMISSIMPLYDUMMYTEXTOFTHEPRINTINGANDTYPESETTINGINDUSTRYLOREMIPSUMHASBEEN\
            THEINDUSTRYSSTANDARDDUMMYTEXTEVERSINCETHEFIFTEENHUNDREDSWHENANUNKNOWNPRINTERTOOKAGALLEYO\
            FTYPEANDSCRAMBLEDITTOMAKEATYPESPECIMENBOOK",
            "FXJZG YKDIT UGTBW EYJWK UAQEF QPIOU PXVSS JDBLM YGKVS XLLRQ IYJDG YGZFW ZXWGF \
            GUTVE JQEWX DDOCR DGPRW EUCUS QRIIC JPTVT KBQUH AZDXT KBARG QQQPB DWTBM DTMIM GPPUI \
            DNWCR LJJTT LZLFB JRSWJ BDIDI LNMBX EBEUH XUPJH ZBZPL XKLGR BCYSE ZWMAS MPRTK WOJVC HHJO"
        );
        test_enigma_i(
            initial_rotor_settings,
            "FXJZGYKDITUGTBWEYJWKUAQEFQPIOUPXVSSJDBLMYGKVSXLLRQIYJDGYGZFWZXWGFGUTVEJQEWXDDOC\
            RDGPRWEUCUSQRIICJPTVTKBQUHAZDXTKBARGQQQPBDWTBMDTMIMGPPUIDNWCRLJJTTLZLFBJRSWJBDIDILNMBXEB\
            EUHXUPJHZBZPLXKLGRBCYSEZWMASMPRTKWOJVCHHJO",
            "LOREM IPSUM ISSIM PLYDU MMYTE XTOFT HEPRI NTING ANDTY PESET TINGI NDUST RYLOR \
            EMIPS UMHAS BEENT HEIND USTRY SSTAN DARDD UMMYT EXTEV ERSIN CETHE FIFTE ENHUN DREDS \
            WHENA NUNKN OWNPR INTER TOOKA GALLE YOFTY PEAND SCRAM BLEDI TTOMA KEATY PESPE CIMEN BOOK"
        );
    }

    mod test_encoding_for_custom_plugboard {
        use super::*;

        #[test]
        fn test_1() {
            let mut plugboard = Plugboard::identity();
            plugboard.connect('A', 'B');
            plugboard.connect('E', 'F');
            plugboard.connect('H', 'I');
            plugboard.connect('N', 'O');
            plugboard.connect('T', 'U');
            plugboard.connect('X', 'Y');

            test_enigma_i_with_custom_plugboard(
                "AAA",
                plugboard,
                "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST",
                "BDFLG OZAUW BRYCN AYNRY TJGEI FFAAJ FEFOS PVKCM NLRYV GKFFK UYQGK YJYBP ZXCZY AGLYF",
            );
        }

        #[test]
        fn test_2() {
            let mut plugboard = Plugboard::identity();
            plugboard.connect('A', 'E');
            plugboard.connect('G', 'L');
            plugboard.connect('Q', 'X');

            test_enigma_i_with_custom_plugboard(
                "EJO",
                plugboard,
                "ABCDEFGHIJKLMNOPQRSTUWXYZZYXWUTSRQPONMLKJIHGFEDCBAABCDEFGHIJKLMNOPQRST",
                "SQBZS UPLCQ JPOXV IBYKH HFRZG HKTAG VZPWI MVEES BHRLR GQFCG PPRLX JCBTN QMBQK EETFX",
            );
        }
    }

    fn test_enigma_i(
        initial_rotor_settings: &str,
        decoded: &str,
        encoded: &str
    ) {
        let plugboard = Plugboard::identity();
        test_enigma_i_with_custom_plugboard(initial_rotor_settings, plugboard, decoded, encoded);
    }

    fn test_enigma_i_with_custom_plugboard(
        initial_rotor_settings: &str,
        plugboard: Plugboard,
        decoded: &str,
        encoded: &str
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

        let mut enigma = Enigma::new(entry_disk, plugboard, rotor_chain, reflector);

        assert_eq!(enigma.encode(decoded), encoded);
    }
}
