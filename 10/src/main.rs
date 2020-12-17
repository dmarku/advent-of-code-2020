use std::collections::HashMap;
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
    println!("lines: {}", input.lines().count());
    let mut joltages: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    joltages.sort();

    println!("part I");

    joltages.insert(0, 0);
    joltages.push(joltages.last().unwrap() + 3);
    let differences: Vec<i32> = joltages
        .iter()
        .skip(1)
        .zip(joltages.iter())
        .map(|(a, b)| a - b)
        .collect();

    let mut bins: HashMap<&i32, u16> = HashMap::new();
    for d in &differences {
        let count = bins.entry(d).or_insert(0);
        *count += 1;
    }

    println!("{}", differences.len());
    println!("{:?}", bins);
    println!(
        "{:?}",
        *bins.get(&1).unwrap_or(&0) * bins.get(&3).unwrap_or(&0)
    );
}
