use std::io::prelude::*;
use std::path::Path;
use std::{fs::File, ops::RangeInclusive};

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

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

fn main() {
    let input = read_input("input.txt");
    println!("{}", input);

    // sections are separated by empty lines
    let mut sections = input.split("\n\n");
    let fields_section = sections.next().unwrap();
    let my_ticket_section = sections.next().unwrap();
    let nearby_tickets_section = sections.next().unwrap();

    fn parse_field(description: &str) -> Option<Field> {
        let mut segments = description.split(": ");

        let name = segments.next()?.to_owned();

        let ranges = segments
            .next()?
            .split(" or ")
            .filter_map(|s| {
                let mut limits = s.split('-');
                let start = limits.next()?.parse::<usize>().ok()?;
                let end = limits.next()?.parse::<usize>().ok()?;
                Some(start..=end)
            })
            .collect::<Vec<_>>();

        Some(Field { name, ranges })
    }

    let fields = fields_section
        .lines()
        .filter_map(parse_field)
        .collect::<Vec<Field>>();

    for f in &fields {
        println!("{:?}", f);
    }

    let tickets: Vec<_> = nearby_tickets_section
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    println!("--- part I ------------------------------------------");

    let error_rate: usize = tickets
        .iter()
        .flatten()
        .filter(|&value| {
            !fields
                .iter()
                .flat_map(|f| &f.ranges)
                .any(|r| r.contains(value))
        })
        .sum();

    println!("the error rate is {:?}", error_rate);

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
