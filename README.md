# Enigma

This project replicates the the behaviour of Enigma - a cipher machine used in the early- to mid-20th century.
More information can be found on its [Wikipedia page](https://en.wikipedia.org/wiki/Enigma_machine).

# Project details

Project contains both library and executable.

**Note:** Library is not "officially" released. At this point it can be used as local dependency only.

## How to use the executable

Build command:
```
cargo build --release
```

Run command (Windows example) to execute from project directory:
```
./target/release/enigma.exe
```

To get full help on required parameters to run the Enigma use `--help`.
Using it you would get the message like this:

```
Enigma 0.1.0

USAGE:
    enigma.exe [FLAGS] [OPTIONS] --basic-position <basic-position> --message <message> --message-key <message-key> --reflector <reflector> --rotor-left <rotor-left> --rotor-middle <rotor-middle> --rotor-right <rotor-right>

FLAGS:
        --allow-cli-questions    Allows CLI questions during runtime to pass all missing, but required, parameters
    -h, --help                   Prints help information
        --use-sample             Use sample Engima as a base for overriding parameters: identity entry disk and
                                 plugboard, reflector B, rotors Enigma I Wehrmacht I, II, III (from left to right)
    -V, --version                Prints version information

OPTIONS:
        --basic-position <basic-position>
            Basic position consisting of three letters, for example: EGW (can be picked at random)

        --message <message>                                  Message to be encoded (max 500 characters)
        --message-key <message-key>
            Message key consisting of three letters, for example: HIB (can be picked at random)

        --plugboard-connection <plugboard-connections>...
            (Optional) Pairs of characters that should be connected in plugboard, for example: AE,BG,GH

        --reflector <reflector>                              Reflector - allowed values: A, B, C
        --rotor-left <rotor-left>                            Left rotor - allowed values: I, II, III, IV, V
        --rotor-middle <rotor-middle>                        Middle rotor - allowed values: I, II, III, IV, V
        --rotor-right <rotor-right>                          Right rotor - allowed values: I, II, III, IV, V
```

Arguments worth mentioning:
- `--allow-cli-questions` - using it will allow (although it will not prevent doing it) to skip the requirement of passing other required arguments, such as `--rotor-left` or `--reflector`. If required argument is not provided, user will be asked to provide missing values during runtime. On the other hand, if required argument was provided, user will not be asked for it.
- `--use-sample` - provides some default values for enigma parts (consult `--help` message for more information). It can be used to simplify the binary execution for tests.
