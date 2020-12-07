use regex::Regex;
use std::format;
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

/*
struct Bag {
    /// the color of this bag
    color: &str,
    /// in which bags it may be contained
    contained_in: mut HashSet<&str>,
    /// which color of bags it can contain and how many
    contains: mut HashMap<&str, u8>,
}
*/

struct Rule<'a> {
    color: String,
    contents: Vec<(&'a str, u8)>,
}

fn parse_rule(rule_text: &str) -> Rule {
    let color: String = rule_text
        .split_whitespace()
        .take(2)
        .collect::<Vec<&str>>()
        .join(" ");

    let contents = vec![("colorless", 25)];

    Rule { color, contents }
}

fn main() {
    let input = read_input("input.txt");
    let rules = input.lines().map(|ref rule| parse_rule(rule));
    for rule in rules {
        let contents_string = rule
            .contents
            .iter()
            .map(|(color, count)| format!("{} of {}", count, color))
            .fold(String::new(), |s, c| s + &c);
        println!("{} contains {}", rule.color, contents_string);
    }
}
