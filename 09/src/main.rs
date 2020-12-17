use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
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

fn find_summand_sequence(numbers: &[u64], number: &u64) -> Option<Range<usize>> {
    for start in 0..numbers.len() {
        for end in start + 1..numbers.len() {
            if numbers[start..end].iter().sum::<u64>() == *number {
                return Some(start..end);
            }
        }
    }

    None
}

fn main() {
    let input = read_input("input.txt");
    let numbers: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

    let preamble_length = 25;

    println!("part I");

    let (i, invalid_number) = numbers
        .iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(i, n)| !is_sum(&numbers[i - preamble_length..*i], &n))
        .expect("found no invalid number");

    println!("first invalid number is {} (#{})", invalid_number, i);

    println!("part II");

    let range = find_summand_sequence(&numbers, &invalid_number)
        .expect("couldn't find a sequence of summands");

    let summands = &numbers[range];
    let sum = summands.iter().min().unwrap() + summands.iter().max().unwrap();

    println!("sum of the smallest and largest in the sequence: {}", sum)
}
