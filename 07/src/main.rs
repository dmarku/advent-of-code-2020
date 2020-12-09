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

        let contents: Vec<(&str, u8)> = if contents_text.starts_with("no other bags") {
            vec![]
        } else {
            contents_text
                .split(", ")
                .map(|item| {
                    let count_string = item.split_whitespace().nth(0).unwrap();
                    let count = count_string.parse::<u8>().ok().unwrap();

                    let color = match (item.find(" "), item.find(" bag")) {
                        (Some(start), Some(end)) => &item[start + 1..end],
                        _ => panic!("unexpected color format {}", item),
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

    // test input
    /*
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";
    */

    let rules: Vec<Rule> = input
        .lines()
        .map(|ref rule| parse_rule(rule).unwrap())
        .collect();

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

    // omit "shiny gold" itself from the results
    println!(
        "PART I: a shiny gold bag may be contained in {} different bags",
        containing_bags.len() - 1
    )
}
