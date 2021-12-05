use std::{io, process};

use enigma::entry_disk::EntryDisk;
use simple_logger::SimpleLogger;

use chrono::Local;
use enigma::enigma::SUPPORTED_ALPHABET;
use enigma::enigma_builder::EnigmaBuilder;
use enigma::message::Message;
use enigma::plugboard::PlugboardConnection;
use enigma::reflector::Reflector;
use enigma::rotors::rotor::Rotor;

use structopt::StructOpt;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Arguments::from_args();

    let mut enigma_builder = if args.use_sample {
        EnigmaBuilder::init()
            .entry_disk(EntryDisk::identity())
            .rotor_left(Rotor::enigma_i_wehrmacht_i())
            .rotor_middle(Rotor::enigma_i_wehrmacht_ii())
            .rotor_right(Rotor::enigma_i_wehrmacht_iii())
            .reflector(Reflector::b())
    } else {
        EnigmaBuilder::init()
    };

    let basic_position: Option<String>;
    let message_key: Option<String>;
    let message_to_encode: Option<String>;

    if args.allow_cli_questions {
        let reflector = args
            .reflector
            .unwrap_or_else(|| read("UKW (reflector)", "Available: A, B or C", REFLECTOR_PARSER));
        enigma_builder = enigma_builder.reflector(reflector);

        let rotor_left = args.rotor_left.unwrap_or_else(|| {
            read(
                "Welzelage I (left rotor)",
                "Available: I, II, III, IV, V",
                ROTOR_PARSER,
            )
        });
        enigma_builder = enigma_builder.rotor_left(rotor_left);

        let rotor_middle = args.rotor_middle.unwrap_or_else(|| {
            read(
                "Welzelage II (middle rotor)",
                "Available: I, II, III, IV, V",
                ROTOR_PARSER,
            )
        });
        enigma_builder = enigma_builder.rotor_middle(rotor_middle);

        let rotor_right = args.rotor_right.unwrap_or_else(|| {
            read(
                "Welzelage III (right rotor)",
                "Available: I, II, III, IV, V",
                ROTOR_PARSER,
            )
        });
        enigma_builder = enigma_builder.rotor_right(rotor_right);

        let plugboard_connections = if args.plugboard_connections.is_empty() {
            read(
                "Steckerverbindungen (plugboard)",
                format!(
                    "Enter pairs of characters that should be connected in plugboard, for example: AE BG GH. Allowed characters: {}",
                    SUPPORTED_ALPHABET
                )
                .as_str(),
                PLUGBOARD_PARSER,
            )
        } else {
            args.plugboard_connections
        };
        enigma_builder = enigma_builder.plugboard_connections(plugboard_connections);

        basic_position = args.basic_position.or_else(|| {
            Some(read(
                "Grundstellung (random basic position)",
                format!(
                    "Random basic position consist of three letters from {}, for example: EGW",
                    SUPPORTED_ALPHABET
                )
                .as_str(),
                BASIC_POSITION_PARSER,
            ))
        });

        message_key = args.message_key.or_else(|| {
            Some(read(
                "Spruchschlussel (random message key)",
                format!(
                    "Random message key consist of three letters from {}, for example: HIB",
                    SUPPORTED_ALPHABET
                )
                .as_str(),
                MESSAGE_KEY_PARSER,
            ))
        });

        message_to_encode = args.message.or_else(|| Some(read(
            "Message to encode",
            "Enter the message that should be encoded using settings provided earlier (max 500 characters)",
            MESSAGE_PARSER
        )));
    } else {
        basic_position = args.basic_position;
        message_key = args.message_key;
        message_to_encode = args.message;
    }

    let mut enigma = enigma_builder.build();

    let encoding_result = enigma.encode(
        basic_position.unwrap(),
        message_key.unwrap(),
        message_to_encode.unwrap(),
    );

    let message = Message::compose(
        Local::now(),
        String::from("REC"),
        String::from("S"),
        encoding_result,
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

    mod cli_arguments {
        use super::*;

        #[test]
        fn works_with_allowing_all_parameters_to_be_provided_in_runtime() {
            assert_eq!(
                Arguments::from_iter(&["test", "--allow-cli-questions",]),
                Arguments {
                    allow_cli_questions: true,
                    use_sample: false,
                    reflector: None,
                    rotor_left: None,
                    rotor_middle: None,
                    rotor_right: None,
                    plugboard_connections: vec![],
                    basic_position: None,
                    message_key: None,
                    message: None
                }
            )
        }

        #[test]
        fn works_with_allowing_all_parameters_to_be_provided_in_runtime_using_sample_enigma() {
            assert_eq!(
                Arguments::from_iter(&["test", "--allow-cli-questions", "--use-sample",]),
                Arguments {
                    allow_cli_questions: true,
                    use_sample: true,
                    reflector: None,
                    rotor_left: None,
                    rotor_middle: None,
                    rotor_right: None,
                    plugboard_connections: vec![],
                    basic_position: None,
                    message_key: None,
                    message: None
                }
            )
        }

        #[test]
        fn using_sample_enigma_requires_providing_encoding_data_but_no_separate_enigma_parts() {
            let no_whitespaces = |s: String| s.split_whitespace().collect::<String>();
            assert_eq!(
                no_whitespaces(
                    Arguments::from_iter_safe(&["test", "--use-sample"]).unwrap_err().message
                ),
                no_whitespaces(String::from(
                    "error: The following required arguments were not provided:\
                        --basic-position <basic-position>\
                        --message <message>\
                        --message-key <message-key>\
                    \
                    USAGE:\
                        test --basic-position <basic-position> --message <message> --message-key <message-key> --use-sample\
                    \
                    For more information try --help"
                ))
            );
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

const PLUGBOARD_CONNECTION_PARSER: fn(&str) -> Result<PlugboardConnection, String> =
    |input: &str| PlugboardConnection::create(input);

const PLUGBOARD_PARSER: fn(&str) -> Result<Vec<PlugboardConnection>, String> = |input: &str| {
    let mut valid: Vec<PlugboardConnection> = vec![];
    let mut errors: Vec<String> = vec![];
    for pair in input.split(',') {
        let result = PLUGBOARD_CONNECTION_PARSER(pair);
        if let Ok(connection) = result {
            valid.push(connection);
        } else {
            errors.push(result.err().unwrap());
        }
    }
    if errors.is_empty() {
        Ok(valid)
    } else {
        Err(errors.join(", "))
    }
};

const SPECIFIC_COUNT_OF_SUPPORTED_ALPHABET_CHARACTERS_PARSER: fn(
    &str,
    u16,
) -> Result<String, String> = |input: &str, required_character_count: u16| {
    let mut errors: Vec<String> = vec![];
    for (i, ch) in input.chars().enumerate() {
        if !SUPPORTED_ALPHABET.contains(ch) {
            errors.push(format!(
                "Character '{}' not found in allowed SUPPORTED_ALPHABET: {}",
                ch, SUPPORTED_ALPHABET
            ));
        }
        if i == required_character_count as usize {
            errors.push(format!(
                "Found more than {} allowed characters",
                required_character_count
            ));
            return Err(errors.join(", "));
        }
    }
    if errors.is_empty() {
        Ok(input.to_string())
    } else {
        Err(errors.join(", "))
    }
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
        errors.push(format!(
            "Input length exceeded allowed limit of {} characters",
            input.len()
        ));
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

fn read<T, C>(title_to_print: &str, info_to_print: &str, read_value_parser: C) -> T
where
    C: Fn(&str) -> Result<T, String>,
{
    println!("|--- {} ---", title_to_print);
    let formatted_info_to_print = format!("| {}", info_to_print);
    println!("{}", formatted_info_to_print);
    println!("| write 'exit' to quit");
    loop {
        let mut v = String::new();
        io::stdin().read_line(&mut v).expect("Failed to read line");
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

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "Enigma")]
struct Arguments {
    #[structopt(
        long = "allow-cli-questions",
        help = "Allows CLI questions during runtime to pass all missing, but required, parameters"
    )]
    allow_cli_questions: bool,

    #[structopt(
        long = "use-sample",
        help = "Use sample Engima as a base for overriding parameters: identity entry disk and plugboard, reflector B, rotors Enigma I Wehrmacht I, II, III (from left to right)"
    )]
    use_sample: bool,

    #[structopt(
		long="reflector",
		help="Reflector - allowed values: A, B, C",
		required_unless_one(&["use-sample", "allow-cli-questions"]),
		parse(try_from_str=REFLECTOR_PARSER)
	)]
    reflector: Option<Reflector>,

    #[structopt(
		long="rotor-left",
		help="Left rotor - allowed values: I, II, III, IV, V",
		required_unless_one(&["use-sample", "allow-cli-questions"]),
		parse(try_from_str=ROTOR_PARSER)
	)]
    rotor_left: Option<Rotor>,

    #[structopt(
		long="rotor-middle",
		help="Middle rotor - allowed values: I, II, III, IV, V",
		required_unless_one(&["use-sample", "allow-cli-questions"]),
		parse(try_from_str=ROTOR_PARSER)
	)]
    rotor_middle: Option<Rotor>,

    #[structopt(
		long="rotor-right",
		help="Right rotor - allowed values: I, II, III, IV, V",
		required_unless_one(&["use-sample", "allow-cli-questions"]),
		parse(try_from_str=ROTOR_PARSER)
	)]
    rotor_right: Option<Rotor>,

    #[structopt(
		long="plugboard-connection",
		help="Pais of characters that should be connected in plugboard, for example: AE BG GH",
		multiple=true,
		parse(try_from_str=PLUGBOARD_CONNECTION_PARSER)
	)]
    plugboard_connections: Vec<PlugboardConnection>,

    #[structopt(
		long="basic-position",
		help="Basic position consisting of three letters, for example: EGW (can be picked at random)",
		required_unless("allow-cli-questions"),
		parse(try_from_str=BASIC_POSITION_PARSER)
	)]
    basic_position: Option<String>,

    #[structopt(
		long="message-key",
		help="Message key consisting of three letters,, for example: HIB (can be picked at random)",
		required_unless("allow-cli-questions"),
		parse(try_from_str=MESSAGE_KEY_PARSER)
	)]
    message_key: Option<String>,

    #[structopt(
		long="message",
		help="Message to be encoded (max 500 characters)",
		required_unless("allow-cli-questions"),
		parse(try_from_str=MESSAGE_PARSER)
	)]
    message: Option<String>,
}
