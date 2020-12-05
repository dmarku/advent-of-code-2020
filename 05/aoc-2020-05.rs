use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::Path;

fn id(seat: &str) -> u16 {
    seat.chars().fold(0, |id, c| {
        (id << 1)
            + match c {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => 0,
            }
    })
}

fn row(id: &u16) -> u16 {
    id >> 3
}

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

    let ids = input.lines().map(|ref seat| id(seat));

    if let Some(highest_id) = ids.clone().max() {
        println!("highest seat ID is {}", highest_id);
    }

    let id_set = HashSet::<u16>::from_iter(ids);
    let unlisted_ids = (0..2u16.pow(10))
        // constraint 1: seat isn't in the list of booked IDs
        .filter(|ref n| !(id_set.contains(*n)))
        // constraint 2: seat isn't in the first or last row
        .filter(|ref n| (row(n) != 0 && row(n) != 127))
        // constraint 3: seats with ID + 1 and ID - 1 are booked
        .filter(|ref n| id_set.contains(&(*n + 1)) && id_set.contains(&(*n - 1)));

    println!("unlisted ids:");

    for id in unlisted_ids {
        println!("{}", id)
    }
}
