use std::{env, io, process};

use simple_logger::SimpleLogger;

use chrono::{Local};
use enigma::reflector::Reflector;
use enigma::rotors::rotor::Rotor;
use enigma::plugboard::PlugboardConnection;
use enigma::enigma_builder::EnigmaBuilder;
use enigma::message::Message;
use enigma::enigma::{SUPPORTED_ALPHABET, Enigma};
use enigma::entry_disk::EntryDisk;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = env::args().collect();
    let init_with_default_enigma = args.contains(&String::from("init-sample"));

    let mut enigma = if init_with_default_enigma {
        println!("Preparing sample Enigma, skipping reading inputs");
        println!();
        EnigmaBuilder::init()
            .entry_disk(EntryDisk::identity())
            .rotor_left(Rotor::enigma_i_wehrmacht_i())
            .rotor_middle(Rotor::enigma_i_wehrmacht_ii())
            .rotor_right(Rotor::enigma_i_wehrmacht_iii())
            .reflector(Reflector::b())
            .build()
    } else {
        create_enigma_from_read_inputs()
    };

    let basic_position = read(
        "Grundstellung (random basic position)",
        "Random basic position consist of three letters from SUPPORTED_ALPHABET, for example: EGW",
        BASIC_POSITION_PARSER
    );
    println!();

    let message_key = read(
        "Spruchschlussel (random message key)",
        "Random basic position consist of three letters from SUPPORTED_ALPHABET, for example: HIB",
        MESSAGE_KEY_PARSER
    );
    println!();

    let message_to_encode = read(
        "Message to encode",
        "Enter the message that should be encoded using settings provided earlier (max 500 characters)",
        MESSAGE_PARSER
    );

    let encoding_result = enigma.encode(
        basic_position,
        message_key,
        message_to_encode
    );

    let message = Message::compose(
        Local::now(),
        String::from("REC"),
        String::from("S"),
        encoding_result
    );

    println!();
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod message_parser {
        use super::*;

        #[test]
        fn valid_message() {
            let result = MESSAGE_PARSER("Lorem Ipsum is simply dummy text");
            assert_eq!(
                result.unwrap(),
                String::from("LOREMXIPSUMXISXSIMPLYXDUMMYXTEXT")
            );
        }

        #[test]
        fn characters_out_of_supported_alphabet() {
            let result = MESSAGE_PARSER("123");
            assert_eq!(
                result.unwrap_err(),
                "Unsupported character '1', \
                Unsupported character '2', \
                Unsupported character '3'"
            )
        }
    }
}

const REFLECTOR_PARSER: fn(&str) -> Result<Reflector, String> = |input: &str| {
    if input.eq("A") {
        Ok(Reflector::a())
    } else if input.eq("B") {
        Ok(Reflector::b())
    } else if input.eq("C") {
        Ok(Reflector::c())
    } else {
        Err(format!("Unsupported reflector type: {}", input))
    }
};

const ROTOR_PARSER: fn(&str) -> Result<Rotor, String> = |input: &str| {
    if input.eq("I") {
        Ok(Rotor::enigma_i_wehrmacht_i())
    } else if input.eq("II") {
        Ok(Rotor::enigma_i_wehrmacht_ii())
    } else if input.eq("III") {
        Ok(Rotor::enigma_i_wehrmacht_iii())
    } else if input.eq("IV") {
        Ok(Rotor::m3_wehrmacht_iv())
    } else if input.eq("V") {
        Ok(Rotor::m3_wehrmacht_v())
    } else {
        Err(format!("Unsupported rotor type: {}", input))
    }
};

const PLUGBOARD_PARSER: fn(&str) -> Result<Vec<PlugboardConnection>, String> = |input: &str| {
    let mut valid: Vec<PlugboardConnection> = vec![];
    let mut errors: Vec<String> = vec![];
    for pair in input.split(' ') {
        let result = PlugboardConnection::create(pair);
        if let Ok(connection) = result {
            valid.push(connection);
        } else {
            errors.push(result.err().unwrap());
        }
    }
    if errors.is_empty() { Ok(valid) } else { Err(errors.join(", ")) }
};

const SPECIFIC_COUNT_OF_SUPPORTED_ALPHABET_CHARACTERS_PARSER: fn(&str, u16) -> Result<String, String> =
    |input: &str, required_character_count: u16| {
        let mut errors: Vec<String> = vec![];
        for (i, ch) in input.chars().enumerate() {
            if !SUPPORTED_ALPHABET.contains(ch) {
                errors.push(format!("Character '{}' not found in allowed SUPPORTED_ALPHABET: {}", ch, SUPPORTED_ALPHABET));
            }
            if i == required_character_count as usize {
                errors.push(format!("Found more than {} allowed characters", required_character_count));
                return Err(errors.join(", "));
            }
        }
        if errors.is_empty() { Ok(input.to_string()) } else { Err(errors.join(", ")) }
    };

const BASIC_POSITION_PARSER: fn(&str) -> Result<String, String> = |input: &str| {
    return SPECIFIC_COUNT_OF_SUPPORTED_ALPHABET_CHARACTERS_PARSER(input, 3);
};

const MESSAGE_KEY_PARSER: fn(&str) -> Result<String, String> = |input: &str| {
    return SPECIFIC_COUNT_OF_SUPPORTED_ALPHABET_CHARACTERS_PARSER(input, 3);
};

const MESSAGE_PARSER: fn(&str) -> Result<String, String> = |input: &str| {
    let mut errors = vec![];
    let mut string_vector = vec![];

    let allowed_limit = 500;
    if input.len() > allowed_limit {
        errors.push(format!("Input length exceeded allowed limit of {} characters", input.len()));
    }

    for c in input.chars() {
        if c.is_whitespace() {
            string_vector.push(String::from('X'));
        } else {
            let uppercase = c.to_uppercase().to_string();
            if SUPPORTED_ALPHABET.contains(&uppercase) {
                string_vector.push(uppercase);
            } else {
                errors.push(format!("Unsupported character '{}'", c));
            }
        }
    }

    if errors.is_empty() {
        Ok(string_vector.join(""))
    } else {
        Err(errors.join(", "))
    }
};

fn read<T, C>(title_to_print: &str,
              info_to_print: &str,
              read_value_parser: C) -> T
    where
        C: Fn(&str) -> Result<T, String> {
    println!("|--- {} ---", title_to_print);
    let formatted_info_to_print = format!("| {}", info_to_print);
    println!("{}", formatted_info_to_print);
    println!("| write 'exit' to quit");
    loop {
        let mut v = String::new();
        io::stdin()
            .read_line(&mut v)
            .expect("Failed to read line");
        let v = v.trim();
        if v.eq("exit") {
            process::exit(0);
        }
        let cv: Result<T, String> = read_value_parser(v);
        if let Ok(x) = cv {
            return x;
        } else {
            println!("| {}", cv.err().unwrap());
            println!("| Write 'exit' to quit or provide valid value");
            println!("{}", formatted_info_to_print);
        }
    }
}

fn create_enigma_from_read_inputs() -> Enigma {
    let mut enigma_builder = EnigmaBuilder::init();

    let reflector = read(
        "UKW (reflector)",
        "reflector: A, B or C",
        REFLECTOR_PARSER);
    enigma_builder = enigma_builder.reflector(reflector);
    println!();

    let rotor_left = read(
        "Welzelage I (left rotor)",
        "Available: I, II, III, IV, V",
        ROTOR_PARSER
    );
    enigma_builder = enigma_builder.rotor_left(rotor_left);

    let rotor_middle = read(
        "Welzelage II (middle rotor)",
        "Available: I, II, III, IV, V",
        ROTOR_PARSER
    );
    enigma_builder = enigma_builder.rotor_middle(rotor_middle);

    let rotor_right = read(
        "Welzelage III (right rotor)",
        "Available: I, II, III, IV, V",
        ROTOR_PARSER
    );
    enigma_builder = enigma_builder.rotor_right(rotor_right);
    println!();

    let plugboard = read(
        "Steckerverbindungen (plugboard)",
        format!(
            "Enter pairs of characters that should be connected in plugboard, \
            for example: AE BG GH. Allowed characters: {}", SUPPORTED_ALPHABET).as_str(),
        PLUGBOARD_PARSER
    );
    enigma_builder = enigma_builder.plugboard_connections(plugboard);
    println!();

    let enigma = enigma_builder.build();
    println!("Enigma machine built properly");
    println!();

    println!("----");
    println!("----");
    println!("----");
    enigma
}
