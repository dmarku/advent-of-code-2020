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

fn main() {
    let input = read_input("input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut program_counter = 0;
    let mut accumulator = 0;

    let mut visited_instructions: HashSet<usize> = HashSet::new();

    while program_counter < lines.len() && !visited_instructions.contains(&program_counter) {
        let line = lines[program_counter];
        visited_instructions.insert(program_counter);
        match &line[..3] {
            "acc" => {
                accumulator += line[4..].parse::<i32>().unwrap();
                program_counter += 1;
            }
            "nop" => {
                program_counter += 1;
            }
            "jmp" => {
                let offset = line[4..].parse::<i32>().unwrap();
                program_counter = if offset.is_negative() {
                    program_counter
                        .checked_sub(offset.wrapping_abs() as u32 as usize)
                        .unwrap()
                } else {
                    program_counter.checked_add(offset as usize).unwrap()
                }
            }
            _ => (),
        }
    }

    print!(
        "program reached duplicate instruction at line {}, accumulator = {}",
        program_counter, accumulator
    );
}
