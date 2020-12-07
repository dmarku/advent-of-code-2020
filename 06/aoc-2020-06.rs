use std::collections::HashSet;
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
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut anyone_yes_total = 0;
    for group in &groups {
        let questionnaires = group.lines();
        let answers_with_yes: HashSet<char> =
            questionnaires.map(|ref q| q.chars()).flatten().collect();
        anyone_yes_total += answers_with_yes.len();
    }

    println!(
        "{} questions were answered by anyone with 'yes'",
        anyone_yes_total
    );

    let mut everyone_yes_total = 0;
    for group in &groups {
        let persons: Vec<&str> = group.lines().collect();
        if let Some((first, rest)) = persons.split_first() {
            let answers_with_yes = first.chars().filter(|label| {
                rest.iter()
                    .all(|ref answers| answers.contains(|l| l == *label))
            });
            everyone_yes_total += answers_with_yes.count();
        }
    }

    println!(
        "{} questions were answered with 'yes' by everyone in any group",
        everyone_yes_total
    );
}
