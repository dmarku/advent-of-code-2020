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
    let (time_string, intervals_string) = {
        let lines: Vec<&str> = input.lines().collect();
        (lines[0], lines[1])
    };

    let time = time_string.parse::<u32>().unwrap();
    let intervals: Vec<_> = intervals_string
        .split(",")
        .filter(|&s| !(&s == &"x"))
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("timestamp = {:?}, intervals = {:?}", time, intervals);

    println!("--- part I ------------------------------------------");

    let (id, wait_time) = intervals
        .iter()
        .map(|i| (i, (time / i + 1) * i - time))
        .min_by_key(|(_, diff)| *diff)
        .unwrap();

    println!(
        "the soonest departure is bus #{} in {} minutes; product = {}",
        id,
        wait_time,
        id * wait_time
    );

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
