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

fn lcm(numbers: &[u32]) -> Result<u32, ()> {
    let mut multiples = Vec::from(numbers);
    // continue as long as multiples aren't equal
    while multiples[1..].iter().any(|&m| m != multiples[0]) {
        let min = *multiples.iter().min().ok_or(())?;
        for (i, m) in multiples.iter_mut().enumerate() {
            if *m == min {
                *m += numbers[i];
            }
        }
    }

    Ok(multiples[0])
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
        .filter_map(|s| s.parse::<u32>().ok())
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

    let intervals_and_offsets: Vec<_> = intervals_string
        .split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse::<usize>().map(|interval| (i, interval)).ok())
        .collect();

    println!("{:?}", intervals_and_offsets);

    // find a t such that (t - offset) % interval == 0 for each (offset, interval) in intervals
    let mut t = intervals_and_offsets[0].1;
    let mut step = intervals_and_offsets[0].1;

    for (raw_offset, interval) in &intervals_and_offsets[1..] {
        let offset = interval - (raw_offset % interval);

        while t % interval != offset {
            t += step;
            /*
            println!(
                "check t = {}; interval/offset = {}/{}; step = {} ",
                t, interval, offset, step
            );
            */
        }

        //println!(" MATCH!");
        step *= interval;
    }

    println!("t = {}", t);
    // known solution for my input
    //assert_eq!(t, 600689120448303);

    // brute force - takes ages
    /*
    let (offset, step) = intervals
        .iter()
        .max_by_key(|(_, interval)| interval)
        .unwrap();

    let t = (1..)
        .map(|i| step * i - *offset as u64)
        .skip_while(|t| {
            intervals
                .iter()
                .any(|(offset, interval)| (t - *offset as u64) % interval != 0)
        })
        .nth(0)
        .unwrap();
    println!("{:?}", t);
    */
}
