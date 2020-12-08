use std::collections::HashSet;
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

// note to self: lifetime annotation <'a> means that a Rule struct cannot exceed
// the lifetime of 'a, so a rule 'dies' lives _at most_ as long as the `color`
// string and any string in its `contents` vector
struct Rule<'a> {
    color: &'a str,
    contents: Vec<(&'a str, u8)>,
}

fn parse_rule(rule_text: &str) -> Option<Rule> {
    if let [container, contents_text] = rule_text.splitn(2, " contain ").collect::<Vec<&str>>()[..2]
    {
        let color_end = container.find("bag")?;
        let color = container[..color_end].trim();

        let contents: Vec<(&str, u8)> = if contents_text.starts_with("no") {
            vec![]
        } else {
            contents_text
                .split(", ")
                .map(|item| {
                    let count_string = item.split_whitespace().nth(0).unwrap();
                    let count = count_string.parse::<u8>().ok().unwrap();

                    let color = match (item.find(" "), item.find(" bag")) {
                        (Some(start), Some(end)) => &item[start + 1..end],
                        _ => "meh",
                    };

                    (color, count)
                })
                .collect()
        };

        Some(Rule { color, contents })
    } else {
        None
    }
}

fn main() {
    let input = read_input("input.txt");

    let rules: Vec<Rule> = input
        .lines()
        .map(|ref rule| parse_rule(rule).unwrap())
        .collect();

    for Rule { color, contents } in &rules {
        if contents.len() > 0 {
            let contents_string = contents
                .iter()
                .map(|(color, count)| format!("{} of {}", count, color))
                .fold(String::new(), |s, c| s + &c + ", ");
            println!("{} contains {}", color, contents_string);
        } else {
            println!("{} is empty", color);
        }
    }

    let mut containing_bags = HashSet::new();
    containing_bags.insert("shiny gold");

    loop {
        let previous_size = containing_bags.len();

        for Rule { color, contents } in &rules {
            if contents
                .iter()
                .any(|(item_color, ..)| containing_bags.contains(item_color))
            {
                containing_bags.insert(color);
            }
        }

        if previous_size >= containing_bags.len() {
            break;
        }
    }

    println!("-------------------------------------");
    for bag in &containing_bags {
        println!("{}", bag);
    }

    // omit "shiny gold" itself from the results
    println!("{}", containing_bags.len() - 1)
}
