#![feature(str_split_once)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => ()
    }

    let mut password_count: usize = 0;
    for line in s.lines() {
        if let Some((policy, password)) = line.split_once(": ") {
            if let Some((range, letter)) = policy.split_once(" ") {
                if let Some((min, max)) = range.split_once("-") {
                    let _min = min.parse::<usize>().unwrap();
                    let _max = max.parse::<usize>().unwrap();
                    if let Some(_letter) = letter.chars().nth(0) {
                        let count = count_letter(&password, &_letter);
                        if count >= _min && count <= _max {
                            password_count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{} passwords are valid", password_count);
}

fn count_letter(string: &str, letter: &char) -> usize {
    string.chars().filter(|c| c == letter).count()
}
