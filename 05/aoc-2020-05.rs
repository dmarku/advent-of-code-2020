use std::fs::File;
use std::io::prelude::*;
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

fn row(id: u16) -> u16 {
    id >> 3
}

fn column(id: u16) -> u16 {
    id & 7
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

    if let Some(highest_id) = ids.max() {
        println!("highest seat ID is {}", highest_id);
    }
}
