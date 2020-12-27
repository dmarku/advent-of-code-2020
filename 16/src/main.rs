use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;
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

#[derive(Debug)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

fn main() {
    let input = read_input("input.txt");

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

    let fields: Vec<Field> = fields_section.lines().filter_map(parse_field).collect();

    let tickets: Vec<Vec<usize>> = nearby_tickets_section
        .lines()
        // skip section header()
        .skip(1)
        .map(|l| l.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    println!("--- part I ------------------------------------------");

    fn value_is_valid(fields: &[Field], value: &usize) -> bool {
        fields
            .iter()
            .flat_map(|f| &f.ranges)
            .any(|r| r.contains(value))
    }

    let error_rate: usize = tickets
        .iter()
        .flatten()
        .filter(|&value| !value_is_valid(&fields, value))
        .sum();

    println!("the error rate is {:?}", error_rate);

    println!("--- part II -----------------------------------------");

    let my_ticket: Vec<usize> = my_ticket_section
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    fn ticket_is_valid(fields: &[Field], ticket: &[usize]) -> bool {
        ticket.iter().all(|value| value_is_valid(fields, value))
    }

    let mut valid_tickets: Vec<&Vec<usize>> = tickets
        .iter()
        .filter(|t| ticket_is_valid(&fields, t))
        .collect();

    valid_tickets.push(&my_ticket);

    // collect values for each position in the tickets
    let mut values_by_field_index: Vec<HashSet<&usize>> = Vec::with_capacity(my_ticket.len());
    for i in 0..my_ticket.len() {
        let values = valid_tickets.iter().map(|t| &t[i]);
        values_by_field_index.push(values.collect());
    }

    let mut indices_by_field: HashMap<&str, HashSet<usize>> = fields
        .iter()
        .map(|f| {
            (
                &f.name[..],
                (0..values_by_field_index.len()).collect::<HashSet<_>>(),
            )
        })
        .collect();

    let fields_by_name: HashMap<&str, &Field> = fields.iter().map(|f| (&f.name[..], f)).collect();

    // first round - filter indices whose values are outside the field's valid ranges
    for (&name, indices) in indices_by_field.iter_mut() {
        let matching_indices = indices.iter().filter_map(|i| {
            if values_by_field_index.get(*i).unwrap().iter().all(|v| {
                fields_by_name
                    .get(name)
                    .unwrap()
                    .ranges
                    .iter()
                    .any(|r| r.contains(v))
            }) {
                Some(*i)
            } else {
                None
            }
        });

        *indices = matching_indices.collect();
    }

    /*
    for entry in &indices_by_field {
        println!("{:?}", entry);
    }
    */

    let mut field_index_map = HashMap::<&str, usize>::new();

    while let Some((name, indices)) = indices_by_field.iter().find(|(_, v)| v.len() == 1) {
        let i = *indices.iter().nth(0).unwrap();
        field_index_map.insert(&name, i);
        for values in indices_by_field.values_mut() {
            values.remove(&i);
        }
    }

    //println!("{:?}", field_index_map);

    let p: usize = field_index_map
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, index)| my_ticket[*index])
        .product();

    println!("product of my ticket's departure field values is {}", p);
}
