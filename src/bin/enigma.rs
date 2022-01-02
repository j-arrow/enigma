use std::{io, process};

use enigma::{entry_disk::EntryDisk, enigma::{EncodingResult, Enigma}, enigma_builder::{BuildError, RotorPlacement}};
use simple_logger::SimpleLogger;

use chrono::Local;
use enigma::enigma::SUPPORTED_ALPHABET;
use enigma::enigma_builder::EnigmaBuilder;
use enigma::plugboard::PlugboardConnection;
use enigma::reflector::Reflector;
use enigma::rotors::rotor::Rotor;

use structopt::StructOpt;

use chrono::DateTime;
use std::fmt::{Display, Error, Formatter};

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

	let mut basic_position = args.basic_position;
	let mut message_key = args.message_key;
	let mut message_to_encode = args.message;

	let mut enigma: Option<Enigma> = None;

	if !args.allow_cli_questions {
		let built = enigma_builder.build()
			.map_err(|build_error| match build_error {
				BuildError::RotorError(rotor_placement, msg) => format!("{} ({} rotor)", msg, rotor_placement),
				BuildError::PlugboardError(msg) => msg,
				BuildError::EntryDiskError(msg) => msg,
				BuildError::ReflectorError(msg) => msg,
			});
		if let Err(err) = built {
			panic!("Unexpected error: {}", err);
		}
		enigma = Some(built.unwrap());
	} else {
		let reflector = args.reflector.unwrap_or_else(read_reflector_from_cli);
		enigma_builder = enigma_builder.reflector(reflector);

		let rotor_left = args.rotor_left.unwrap_or_else(read_left_rotor_from_cli);
		enigma_builder = enigma_builder.rotor_left(rotor_left);

		let rotor_middle = args.rotor_middle.unwrap_or_else(read_middle_rotor_from_cli);
		enigma_builder = enigma_builder.rotor_middle(rotor_middle);

		let rotor_right = args.rotor_right.unwrap_or_else(read_right_rotor_from_cli);
		enigma_builder = enigma_builder.rotor_right(rotor_right);

		let plugboard_connections = if args.plugboard_connections.is_empty() {
			read_plugboard_connections_from_cli()
		} else {
			args.plugboard_connections
				.into_iter()
				.map(|c| match c {
					PlugboardConnectionOption::Existing(existing) => existing,
					PlugboardConnectionOption::None => panic!("Must not happen"),
				})
				.collect()
		};
		enigma_builder = enigma_builder.plugboard_connections(plugboard_connections);

		if let None = basic_position {
			basic_position = Some(read_basic_position_from_cli())
		}

		if let None = message_key {
			message_key = Some(read_message_key_from_cli())
		}

		if let None = message_to_encode {
			message_to_encode = Some(read_message_to_encode_from_cli())
		}

		let mut en = enigma_builder.build();
		while let Err(build_error) = en {
			println!();
			match build_error {
				BuildError::RotorError(rotor_placement, err) => {
					eprintln!("Rotor ({}) error when building Enigma: {}", rotor_placement, err);
					match rotor_placement {
						RotorPlacement::Left => enigma_builder = enigma_builder.rotor_left(read_left_rotor_from_cli()),
						RotorPlacement::Middle => enigma_builder = enigma_builder.rotor_middle(read_middle_rotor_from_cli()),
						RotorPlacement::Right => enigma_builder = enigma_builder.rotor_right(read_right_rotor_from_cli()),
					}
					en = enigma_builder.build();
				},
				BuildError::PlugboardError(err) => {
					eprintln!("Plugboard error when building Enigma: {}", err);
					enigma_builder = enigma_builder.plugboard_connections(read_plugboard_connections_from_cli());
					en = enigma_builder.build();
				},
				BuildError::EntryDiskError(err) => {
					todo!("Entry disk reading from CLI is not implemented yet: {}", err);
				},
				BuildError::ReflectorError(err) => {
					eprintln!("Reflector error when building Enigma: {}", err);
					enigma_builder = enigma_builder.reflector(read_reflector_from_cli());
					en = enigma_builder.build();
				},
			}
		}
		enigma = Some(en.unwrap());
	}

	let basic_position = basic_position.unwrap();
	let message_key = message_key.unwrap();

	let mut enigma = enigma.unwrap();

    let mut encoding_result = enigma.encode(
        basic_position.clone(),
        message_key.clone(),
        message_to_encode.unwrap(),
    );

	while let Err(err) = encoding_result {
		eprintln!("Failed to encode the message due to error:");
		eprintln!("{}", err);
		println!("Provide new message to encode.");

		message_to_encode = Some(read(
            "Message to encode",
            "Enter the message that should be encoded using settings provided earlier (max 500 characters)",
            MESSAGE_PARSER
        ));

		encoding_result = enigma.encode(
			basic_position.clone(),
			message_key.clone(),
			message_to_encode.unwrap(),
		);
	}

    let message = Message::compose(
        Local::now(),
        String::from("REC"),
        String::from("S"),
        encoding_result.unwrap(),
    );

    println!();
    println!("{}", message);
}

fn read_reflector_from_cli() -> Reflector {
	read("UKW (reflector)", "Available: A, B or C", REFLECTOR_PARSER)
}
fn read_left_rotor_from_cli() -> Rotor {
	read(
		"Welzelage I (left rotor)",
		"Available: I, II, III, IV, V",
		ROTOR_PARSER,
	)
}
fn read_middle_rotor_from_cli() -> Rotor {
	read(
		"Welzelage II (middle rotor)",
		"Available: I, II, III, IV, V",
		ROTOR_PARSER,
	)
}
fn read_right_rotor_from_cli() -> Rotor {
	read(
		"Welzelage III (right rotor)",
		"Available: I, II, III, IV, V",
		ROTOR_PARSER,
	)
}
fn read_plugboard_connections_from_cli() -> Vec<PlugboardConnection> {
	read(
		"Steckerverbindungen (plugboard)",
		format!(
			"(Optional, press 'enter' key if not required) Enter pairs of characters that should be connected in plugboard (pairs must be split by comma), for example: AE,BG,GH. Allowed characters: {}",
			SUPPORTED_ALPHABET
		)
		.as_str(),
		PLUGBOARD_PARSER,
	)
}
fn read_basic_position_from_cli() -> String {
	read(
		"Grundstellung (random basic position)",
		format!(
			"Random basic position consist of three letters from {}, for example: EGW",
			SUPPORTED_ALPHABET
		)
		.as_str(),
		BASIC_POSITION_PARSER,
	)
}
fn read_message_key_from_cli() -> String {
	read(
		"Spruchschlussel (random message key)",
		format!(
			"Random message key consist of three letters from {}, for example: HIB",
			SUPPORTED_ALPHABET
		)
		.as_str(),
		MESSAGE_KEY_PARSER,
	)
}
fn read_message_to_encode_from_cli() -> String {
	read(
		"Message to encode",
		"Enter the message that should be encoded using settings provided earlier (max 500 characters)",
		MESSAGE_PARSER
	)
}

pub struct Message {
    message_time: String,
    receiver: String,
    sender: String,
    message_length: usize,
    basic_position: String,
    encoded_message_key: String,
    encoded_message: String,
}

impl Message {
    pub fn compose(
        message_time: DateTime<Local>,
        receiver: String,
        sender: String,
        encoding_result: EncodingResult,
    ) -> Self {
        Message {
            message_time: message_time.format("%H%M").to_string(),
            receiver,
            sender,
            message_length: encoding_result.message_length,
            basic_position: encoding_result.basic_position,
            encoded_message_key: encoding_result.encoded_message_key,
            encoded_message: encoding_result.encoded_message,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // for example: U6Z DE C 1510 = 49 = EHZ TBS = TVEXS QBLTW LDAHH YEOEF
        // means: message sent from C to D6Z on 15:10, message is 49 characters long; basic position is EHZ,
        // encrypted message key is TBS, identification group for a day is EVEXS and the rest is encoded
        // message split with space

        let encoded_message = self
            .encoded_message
            .chars()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f,
               "{receiver} DE {sender} {sending_time} = {message_length} = {basic_position} {encrypted_message_key} = {identification_group} {encoded_message}",
               receiver = self.receiver,
               sender = self.sender,
               sending_time = self.message_time,
               message_length = self.message_length,
               basic_position = self.basic_position,
               encrypted_message_key = self.encoded_message_key,
               identification_group = "ABCDE",
               encoded_message = encoded_message
        )
    }
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
                String::from("LOREM IPSUM IS SIMPLY DUMMY TEXT")
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

		#[test]
		fn too_long_message_to_process() {
			let result = MESSAGE_PARSER(&"a".repeat(501));
			assert_eq!(
				result.unwrap_err(),
				"Input length exceeded allowed limit of 500, as it contains 501 characters"
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

#[derive(PartialEq, Debug)]
enum PlugboardConnectionOption {
	Existing(PlugboardConnection),
	None
}

const PLUGBOARD_CONNECTION_PARSER: fn(&str) -> Result<PlugboardConnectionOption, String> = |input: &str| {
	if input.chars().count() == 0 {
		Ok(PlugboardConnectionOption::None)
	} else {
		PlugboardConnection::create(input)
			.map(|con| PlugboardConnectionOption::Existing(con))
	}
};

const PLUGBOARD_PARSER: fn(&str) -> Result<Vec<PlugboardConnection>, String> = |input: &str| {
    let mut valid: Vec<PlugboardConnection> = vec![];
    let mut errors: Vec<String> = vec![];
    for pair in input.split(',') {
        let result = PLUGBOARD_CONNECTION_PARSER(pair);
        if let Ok(connection) = result {
			if let PlugboardConnectionOption::Existing(con) = connection {
            	valid.push(con);
			}
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
            "Input length exceeded allowed limit of {}, as it contains {} characters",
            allowed_limit, input.len()
        ));
    }

    for c in input.chars() {
        if c.is_whitespace() {
            // ignore, whitespaces for now - those will be handled during encoding
			string_vector.push(String::from(" "));
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
		match read_value_parser(v) {
			Ok(x) => return x,
			Err(err) => {
				eprintln!("| {}", err);
				eprintln!("| Write 'exit' to quit or provide valid value");
				eprintln!("{}", formatted_info_to_print);
			},
		};
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
		help="(Optional) Pairs of characters that should be connected in plugboard, for example: AE,BG,GH",
		multiple=true,
		parse(try_from_str=PLUGBOARD_CONNECTION_PARSER)
	)]
    plugboard_connections: Vec<PlugboardConnectionOption>,

    #[structopt(
		long="basic-position",
		help="Basic position consisting of three letters, for example: EGW (can be picked at random)",
		required_unless("allow-cli-questions"),
		parse(try_from_str=BASIC_POSITION_PARSER)
	)]
    basic_position: Option<String>,

    #[structopt(
		long="message-key",
		help="Message key consisting of three letters, for example: HIB (can be picked at random)",
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
