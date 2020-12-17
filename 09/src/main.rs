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

fn is_sum(numbers: &[u64], value: &u64) -> bool {
    for a in numbers {
        for b in numbers {
            if a == b {
                continue;
            }
            if a + b == *value {
                return true;
            }
        }
    }

    false
}

fn main() {
    let input = read_input("input.txt");
    let numbers: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

    let preamble_length = 25;

    let checked: Vec<_> = numbers
        .iter()
        .enumerate()
        .skip(preamble_length)
        .map(|(i, n)| (i, n, is_sum(&numbers[i - preamble_length..i], &n)))
        .collect();

    println!("{:?}", checked);
}
