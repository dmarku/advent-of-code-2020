use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

enum Stop {
    Terminated(i32),
    LoopDetected(usize, i32),
}

#[derive(Clone, Copy)]
enum Instruction {
    Accumulate(i32),
    Nop(i32),
    Jump(i32),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Accumulate(value) => write!(f, "acc {}", value),
            Instruction::Jump(offset) => write!(f, "jmp {}", offset),
            Instruction::Nop(value) => write!(f, "nop {}", value),
        }
    }
}

fn decode(instruction: &str) -> Instruction {
    let opcode = &instruction[..3];
    match opcode {
        "acc" => Instruction::Accumulate(instruction[4..].parse::<i32>().unwrap()),
        "nop" => Instruction::Nop(instruction[4..].parse::<i32>().unwrap()),
        "jmp" => Instruction::Jump(instruction[4..].parse::<i32>().unwrap()),
        _ => panic!("invalid instruction"),
    }
}

fn run(instructions: &Vec<Instruction>) -> Result<Stop, ()> {
    let mut program_counter = 0;
    let mut accumulator = 0;

    let mut visited_instructions: HashSet<usize> = HashSet::new();

    while program_counter < instructions.len() {
        if visited_instructions.contains(&program_counter) {
            return Ok(Stop::LoopDetected(program_counter, accumulator));
        }
        visited_instructions.insert(program_counter);

        let ref instruction = instructions[program_counter];
        match instruction {
            Instruction::Accumulate(value) => {
                accumulator += value;
                program_counter += 1;
            }
            Instruction::Nop(_) => {
                program_counter += 1;
            }
            Instruction::Jump(offset) => {
                program_counter = if offset.is_negative() {
                    program_counter
                        .checked_sub(offset.wrapping_abs() as usize)
                        .unwrap()
                } else {
                    program_counter.checked_add(*offset as usize).unwrap()
                };
            }
        }
    }

    Ok(Stop::Terminated(accumulator))
}

fn main() {
    let input = read_input("input.txt");
    /*let input = "nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6";
    */

    let instructions: Vec<Instruction> = input.lines().map(|line| decode(line)).collect();

    println!("Part I");

    match run(&instructions) {
        Ok(Stop::Terminated(accumulator)) => {
            println!("program terminated with accumulator = {}", accumulator)
        }
        Ok(Stop::LoopDetected(program_counter, accumulator)) => println!(
            "program reached duplicate instruction at line {}, accumulator = {}",
            program_counter, accumulator
        ),
        _ => (),
    }

    println!("Part II");

    // brute force approach - replace any single nop/jmp, see if program execution terminates
    for line_no in 0..instructions.len() {
        let modified_instructions: Vec<Instruction> = instructions
            .iter()
            .enumerate()
            .map(|(n, i)| {
                if n == line_no {
                    match i {
                        Instruction::Nop(v) => Instruction::Jump(*v),
                        Instruction::Jump(offset) => Instruction::Nop(*offset),
                        _ => *i,
                    }
                } else {
                    *i
                }
            })
            .collect();

        match run(&modified_instructions).unwrap() {
            Stop::Terminated(accumulator) => println!(
                "successfully finished with modifying line #{} ({} -> {}); accu = {}",
                line_no, instructions[line_no], modified_instructions[line_no], accumulator
            ),
            Stop::LoopDetected(pc, acc) => {
                //println!("program loops at {}; last accumulator: {}", pc, acc)
            }
        }
    }
}
