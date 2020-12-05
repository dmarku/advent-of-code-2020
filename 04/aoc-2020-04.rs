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

    let documents: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|document| {
            let mut props = HashMap::with_capacity(8);
            for property in document.split_whitespace() {
                if let [key, value] = property.split(":").collect::<Vec<&str>>()[..2] {
                    props.insert(key, value);
                }
            }

            props
        })
        .collect();

    let required_properties = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let has_all_props = |document: &HashMap<&str, &str>| {
        required_properties
            .iter()
            .all(|property| document.contains_key(property))
    };

    let loosely_valid_doc_count = documents
        .iter()
        .filter(|ref doc| has_all_props(doc))
        .count();
    println!("{} loosely valid documents", loosely_valid_doc_count);

    let strictly_valid_doc_count = &documents
        .iter()
        .filter(|ref document| {
            has_all_props(document)
                && document.iter().all(|(key, value)| match *key {
                    "byr" => {
                        value.len() == 4
                            && match value.parse::<u16>() {
                                Ok(year) => year >= 1920 && year <= 2002,
                                Err(_) => false,
                            }
                    }
                    "iyr" => {
                        value.len() == 4
                            && match value.parse::<u16>() {
                                Ok(year) => year >= 2010 && year <= 2020,
                                Err(_) => false,
                            }
                    }
                    "eyr" => {
                        value.len() == 4
                            && match value.parse::<u16>() {
                                Ok(year) => year >= 2020 && year <= 2030,
                                Err(_) => false,
                            }
                    }
                    "hgt" => {
                        (value.ends_with("cm")
                            && match value[..value.len() - 2].parse::<u8>() {
                                Ok(height) => (height >= 150 && height <= 193),
                                Err(_) => false,
                            })
                            || (value.ends_with("in")
                                && match value[..value.len() - 2].parse::<u8>() {
                                    Ok(height) => (height >= 59 && height <= 76),
                                    Err(_) => false,
                                })
                            || (false)
                    }
                    "hcl" => {
                        value.len() == 7
                            && value.chars().nth(0) == Some('#')
                            && value
                                .chars()
                                .skip(1)
                                .all(|c| "0123456789abdcef".contains(c))
                    }
                    "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
                    "pid" => value.len() == 9 && value.chars().all(|c| "0123456789".contains(c)),
                    _ => true,
                })
        })
        .count();

    println!("{} strictly valid documents", strictly_valid_doc_count);
}
