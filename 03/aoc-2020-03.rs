use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

enum Tile {
    Open,
    Tree,
    Unknown,
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;
    for (inc_x, inc_y) in slopes {
        let tree_count = check_slope(&s, inc_x, inc_y);
        println!("Right {}, down {}: {} trees", inc_x, inc_y, tree_count);
        product *= tree_count;
    }

    println!("tree count product: {}", product);
}

fn parse_character(c: &char) -> Tile {
    match c {
        '.' => Tile::Open,
        '#' => Tile::Tree,
        _ => Tile::Unknown,
    }
}

fn check_slope(s: &str, increment_x: usize, increment_y: usize) -> usize {
    // current position
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;
    for line in s.lines() {
        if (y % increment_y) == 0 {
            if let Some(Tile::Tree) = line
                .chars()
                .map(|c| parse_character(&c))
                .nth(x % line.len())
            {
                tree_count += 1;
            }
            x += increment_x;
        }
        y += 1;
    }

    tree_count
}
