use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
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

    let documents = input.split("\n\n").map(|document| {
        let mut props = HashMap::with_capacity(8);
        for property in document.split_whitespace() {
            if let [key, value] = property.split(":").collect::<Vec<&str>>()[..2] {
                props.insert(key, value);
            }
        }

        props
    });

    let required_properties = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let loosely_valid_doc_count = documents
        .filter(|ref document| {
            required_properties.iter().fold(true, |valid, property| {
                valid && document.contains_key(property)
            })
        })
        .count();

    println!("{} valid documents", loosely_valid_doc_count);
}
