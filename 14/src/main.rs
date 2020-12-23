use std::io::prelude::*;
use std::path::Path;
use std::{collections::HashMap, fs::File};

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
    //let input = read_input("input.txt");
    let input = read_input("input_example.txt");
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

    for line in input.lines() {
        if line.starts_with("mask = ") {
            let bits = line.strip_prefix("mask = ").unwrap();
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
        } else if line.starts_with("mem[") {
            if let (Some(start), Some(end)) = (line.find('['), line.find(']')) {
                let address: &usize = &line[start + 1..end].parse().unwrap();
                let value: u64 = line
                    .find(" = ")
                    .and_then(|i| Some(&line[i + 3..]))
                    .and_then(|s| s.parse().ok())
                    .unwrap();
                state
                    .memory
                    .insert(*address, value & state.mask_0 | state.mask_1);
            }
        }
    }

    let sum: u64 = state.memory.values().sum();
    println!("value sum is {}", sum);

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
