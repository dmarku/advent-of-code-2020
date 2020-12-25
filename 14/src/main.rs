use std::io::prelude::*;
use std::path::Path;
use std::{collections::HashMap, fs::File};
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

fn read_input(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    input
}

fn main() {
    let input = read_input("input.txt");
    //let input = read_input("input_example.txt");
    println!("{}", input);

    println!("--- part I ------------------------------------------");
    struct State {
        mask_0: u64,
        mask_1: u64,
        memory: HashMap<usize, u64>,
    }

    let mut state = State {
        mask_0: !0,
        mask_1: 0,
        memory: HashMap::new(),
    };

    enum Instruction {
        Mask(String),
        Mem { address: usize, value: u64 },
    }

    #[derive(Error, Debug)]
    enum InstructionError {
        #[error("wrong 'mask' instruction format")]
        WrongMask,
        #[error("wrong 'mem' instruction format")]
        WrongMem,
        #[error("unknown instruction")]
        UnknownInstruction,
        #[error(transparent)]
        ParseError(#[from] ParseIntError),
    }

    impl FromStr for Instruction {
        type Err = InstructionError;

        fn from_str(line: &str) -> Result<Instruction, InstructionError> {
            if line.starts_with("mask = ") {
                let bits = line
                    .strip_prefix("mask = ")
                    .ok_or(InstructionError::WrongMask)?;
                return Ok(Instruction::Mask(bits.to_owned()));
            } else if line.starts_with("mem[") {
                if let (Some(start), Some(end)) = (line.find('['), line.find(']')) {
                    let address: &usize = &line[start + 1..end].parse()?;
                    let value_offset = 3 + line.find(" = ").ok_or(InstructionError::WrongMem)?;
                    let value: u64 = (&line[value_offset..]).parse()?;

                    return Ok(Instruction::Mem {
                        address: *address,
                        value,
                    });
                } else {
                    return Err(InstructionError::WrongMem);
                }
            } else {
                return Err(InstructionError::UnknownInstruction);
            }
        }
    }

    for line in input.lines() {
        match line.parse::<Instruction>() {
            Ok(Instruction::Mask(bits)) => {
                state.mask_1 = bits.chars().fold(0, |n, c| {
                    (n << 1)
                        | match c {
                            '1' => 1,
                            _ => 0,
                        }
                });
                state.mask_0 = bits.chars().fold(!0, |n, c| {
                    (n << 1)
                        | match c {
                            '0' => 0,
                            _ => 1,
                        }
                });
            }
            Ok(Instruction::Mem { address, value }) => {
                state
                    .memory
                    .insert(address, value & state.mask_0 | state.mask_1);
            }
            _ => (),
        }
    }

    let sum: u64 = state.memory.values().sum();
    println!("value sum is {}", sum);

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
