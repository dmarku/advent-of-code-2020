use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
    let expenses = s.lines().map(|line| line.parse::<u32>().unwrap());

    let pair = expenses
        .map(|a: &u32| expenses.map(|b: &u32| (a, b)))
        .flatten()
        .filter(|(a, b)| a != b && a + b == 2020)
        .nth(0);

    match pair {
        Some((a, b)) => println!("matching pair is {}, {}; product is {}", &a, &b, &a * &b),
        None => println!("couldn't find a matching pair"),
    }

    let triple = expenses
        .map(|a: &u32| expenses.map(|b: &u32| expenses.map(|c: &u32| (a, b, c))).flatten())
        .flatten()
        .filter(|(a, b, c)| a != b && b != c && c != a &&  a + b + c == 2020)
        .nth(0);

    match triple {
        Some((a, b, c)) => println!("matching pair is {}, {}, {}; product is {}", &a, &b, &c,  &a * &b * &c),
        None => println!("couldn't find a matching pair"),
    }
}
