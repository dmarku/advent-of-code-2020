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

    let mut password_count_v1: usize = 0;
    for line in s.lines() {
        match parse_line(&line) {
            Some(Entry {min, max, letter, password}) => {
                let count = count_letter(&password, &letter);
                if count >= min && count <= max {
                    password_count_v1 += 1;
                }
            },
            None => (),
        }
    }

    println!("{} passwords are valid according to part 1", password_count_v1);
}

struct Entry<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn parse_line(line: &str) -> Option<Entry> {
    let (policy, password) = line.split_once(": ")?;
    let (range, letter) = policy.split_once(" ")?;
    let (min, max) = range.split_once("-")?;
    let _min = min.parse::<usize>().ok()?;
    let _max = max.parse::<usize>().ok()?;
    let _letter = letter.chars().nth(0)?;

    Some(Entry { min: _min, max: _max, letter: _letter, password })
}


fn count_letter(string: &str, letter: &char) -> usize {
    string.chars().filter(|c| c == letter).count()
}
