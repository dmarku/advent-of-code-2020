use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
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
    //let input = read_input("input_example_2.txt");
    println!("{}", input);

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

    struct State2 {
        mask_ones: usize,
        floating_positions: Vec<usize>,
        memory: HashMap<usize, u64>,
    }

    let mut state = State2 {
        mask_ones: 0,
        floating_positions: vec![],
        memory: HashMap::new(),
    };

    fn floating_positions(mask: &str) -> Vec<usize> {
        mask.chars()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
            .collect()
    }

    fn variations(positions: &[usize], address: usize) -> Box<dyn Iterator<Item = usize>> {
        if let Some((p, ps)) = positions.split_first() {
            let mask = !(1 << p);
            let zero = address & mask;
            let one = address & mask | (1 << p);
            Box::new(variations(ps, zero).chain(variations(ps, one)))
        } else {
            Box::new(once(address))
        }
    }

    for line in input.lines() {
        match line.parse::<Instruction>() {
            Ok(Instruction::Mask(bits)) => {
                state.mask_ones = bits.chars().fold(0, |n, c| {
                    (n << 1)
                        | match c {
                            '1' => 1,
                            _ => 0,
                        }
                });
                state.floating_positions = floating_positions(&bits);
            }
            Ok(Instruction::Mem { address, value }) => {
                // TODO: expand addresses
                for addr in variations(&state.floating_positions, address | state.mask_ones) {
                    state.memory.insert(addr, value);
                }
            }
            _ => (),
        }
    }

    let sum: u64 = state.memory.values().sum();
    println!("value sum is {}", sum);
}
