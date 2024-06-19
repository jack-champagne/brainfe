// a brainf*ck interpreter

use std::{
    env, fs,
    io::{self, Read},
    path::Path,
    process,
};

#[derive(PartialEq, Clone)]
enum Token {
    IncrementDataPointer,
    DecrementDataPointer,
    IncrementByte,
    DecrementByte,
    OutputByte,
    InputByte,
    LeftBracket,
    RightBracket,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let brainfe_program = get_program_string(args);
    let tokens = program_to_tokens(brainfe_program);
    let mut tape = Vec::<u8>::new();
    tape.resize(30_000, 0);

    run_program(tape, tokens);
}

fn program_to_tokens(brainfe_program: String) -> Vec<Token> {
    brainfe_program
        .chars()
        .map(|c| match c {
            '>' => Token::IncrementDataPointer,
            '<' => Token::DecrementDataPointer,
            '+' => Token::IncrementByte,
            '-' => Token::DecrementByte,
            '.' => Token::OutputByte,
            ',' => Token::InputByte,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            c => {
                println!("Invalid character {c} in program.");
                process::exit(1);
            }
        })
        .collect()
}

fn run_program(mut tape: Vec<u8>, tokens: Vec<Token>) {
    let mut instruction_pointer = 0;
    let mut data_pointer = 0;
    while instruction_pointer < tokens.len() {
        match tokens[instruction_pointer] {
            Token::IncrementDataPointer => {
                if data_pointer == tape.len() {
                    // really stupid but memor minimal as possible.
                    tape.resize(tape.len() + 1, 0);
                }
                data_pointer += 1;
            }
            Token::DecrementDataPointer => {
                if data_pointer == 0 {
                    panic!("data pointer became negative:\n{instruction_pointer}");
                }
                data_pointer -= 1;
            }
            Token::IncrementByte => {
                if tape[data_pointer] == 255 {
                    tape[data_pointer] = 0;
                } else {
                    tape[data_pointer] += 1;
                }
            } 
            Token::DecrementByte => {
                if tape[data_pointer] == 0 {
                    tape[data_pointer] = 255;
                } else {
                    tape[data_pointer] -= 1;
                }
            } 
            Token::OutputByte => print!("{}", tape[data_pointer] as char),
            Token::InputByte => match io::stdin().bytes().next() {
                Some(Ok(c)) => tape[data_pointer] = c,
                Some(Err(_)) => panic!("could not parse character"),
                None => panic!("reached EOF"),
            },
            Token::LeftBracket => {
                if tape[data_pointer] == 0 {
                    // skip to corresponding ']' or right bracket
                    let (offset, balance) = tokens[instruction_pointer..]
                        .iter()
                        .enumerate()
                        .map(|(i, e)| match *e {
                            Token::RightBracket => (i, -1),
                            Token::LeftBracket => (i, 1),
                            _ => (i, 0),
                        })
                        .skip(1)
                        .fold(
                            (0, 1),
                            |(i, v), (index, counter)| {
                                if v == 0 {
                                    (i, v)
                                } else {
                                    (index, v + counter)
                                }
                            },
                        );
                    if balance != 0 {
                        panic!("Unmatched [")
                    } else {
                        instruction_pointer += offset
                    }
                }
            }
            Token::RightBracket => {
                if tape[data_pointer] != 0 {
                    // skip to corresponding ']' or right bracket
                    let search_slice = &mut tokens.clone()[0..=instruction_pointer];
                    search_slice.reverse();

                    let (offset, balance) = search_slice
                        .iter()
                        .enumerate()
                        .map(|(i, e)| match *e {
                            Token::RightBracket => (i, 1),
                            Token::LeftBracket => (i, -1),
                            _ => (i, 0),
                        })
                        .skip(1)
                        .fold(
                            (0, 1),
                            |(i, v), (index, counter)| {
                                if v == 0 {
                                    (i, v)
                                } else {
                                    (index, v + counter)
                                }
                            },
                        );
                    if balance != 0 {
                        panic!("Unmatched ]")
                    } else {
                        instruction_pointer -= offset
                    }
                }
            }
        }
        instruction_pointer += 1;
    }
}

fn get_program_string(args: Vec<String>) -> String {
    if args.len() < 1 {
        println!("Not enough arguments")
    }

    let filename = &args[1];
    match Path::try_exists(Path::new(filename)) {
        Ok(true) => fs::read_to_string(filename).expect("could not read contents of file"),
        Ok(false) => {
            println!("Not enough arguments");
            process::exit(1);
        }
        Err(_) => {
            println!("Not enough arguments");
            process::exit(1);
        }
    }
}
