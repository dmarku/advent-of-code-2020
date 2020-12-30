use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_text(s: &str) -> Option<Vec<Point>> {
    let mut points = Vec::new();
    for (y, l) in s.lines().enumerate() {
        let y = i32::try_from(y).ok()?;
        for (x, c) in l.chars().enumerate() {
            let x = i32::try_from(x).ok()?;
            if c == '#' {
                points.push(Point { x, y, z: 0 });
            }
        }
    }

    Some(points)
}

enum Liveliness {
    Alive,
    Dead,
}

#[rustfmt::skip]
fn env_xyz(p: &Point) -> IntoIter<Point> {
    vec![
        // axis neighbors
        Point { x: p.x - 1, ..*p },
        Point { x: p.x + 1, ..*p },

        Point { y: p.y - 1, ..*p },
        Point { y: p.y + 1, ..*p },

        Point { z: p.z - 1, ..*p },
        Point { z: p.z + 1, ..*p },

        // planar neighbors
        Point { x: p.x - 1, y: p.y - 1, ..*p },
        Point { x: p.x + 1, y: p.y - 1, ..*p },
        Point { x: p.x - 1, y: p.y + 1, ..*p },
        Point { x: p.x + 1, y: p.y + 1, ..*p },

        Point { y: p.y - 1, z: p.z - 1, ..*p },
        Point { y: p.y + 1, z: p.z - 1, ..*p },
        Point { y: p.y - 1, z: p.z + 1, ..*p },
        Point { y: p.y + 1, z: p.z + 1, ..*p },

        Point { z: p.z - 1, x: p.x - 1, ..*p },
        Point { z: p.z + 1, x: p.x - 1, ..*p },
        Point { z: p.z - 1, x: p.x + 1, ..*p },
        Point { z: p.z + 1, x: p.x + 1, ..*p },

        // diagonal neighbors
        Point { x: p.x - 1, y: p.y - 1, z: p.z - 1 },
        Point { x: p.x + 1, y: p.y - 1, z: p.z - 1 },
        Point { x: p.x - 1, y: p.y + 1, z: p.z - 1 },
        Point { x: p.x + 1, y: p.y + 1, z: p.z - 1 },
        Point { x: p.x - 1, y: p.y - 1, z: p.z + 1 },
        Point { x: p.x + 1, y: p.y - 1, z: p.z + 1 },
        Point { x: p.x - 1, y: p.y + 1, z: p.z + 1 },
        Point { x: p.x + 1, y: p.y + 1, z: p.z + 1 },
    ].into_iter()
}

fn state(cells: &HashSet<Point>, cell: &Point) -> Liveliness {
    if cells.contains(cell) {
        Liveliness::Alive
    } else {
        Liveliness::Dead
    }
}

fn next_state(l: Liveliness, neighbors: usize) -> Liveliness {
    match l {
        Liveliness::Alive => match neighbors {
            n if n == 2 || n == 3 => Liveliness::Alive,
            _ => Liveliness::Dead,
        },
        Liveliness::Dead => match neighbors {
            n if n == 3 => Liveliness::Alive,
            _ => Liveliness::Dead,
        },
    }
}

fn step(cells: &HashSet<Point>) -> HashSet<Point> {
    cells
        .iter()
        .flat_map(|c| env_xyz(c))
        .fold(HashMap::new(), |mut liveliness, cell| {
            *liveliness.entry(cell).or_insert(0) += 1;
            liveliness
        })
        .into_iter()
        .filter(
            |(p, neighbors)| match next_state(state(cells, p), *neighbors) {
                Liveliness::Alive => true,
                Liveliness::Dead => false,
            },
        )
        .map(|(p, _)| p)
        .collect::<HashSet<_>>()
}

fn part_1(input: &str) -> usize {
    let temp_cells = parse_text(&input).unwrap();
    let cells: HashSet<Point> = temp_cells.into_iter().collect();

    let gen_1 = step(&cells);
    let gen_2 = step(&gen_1);
    let gen_3 = step(&gen_2);
    let gen_4 = step(&gen_3);
    let gen_5 = step(&gen_4);
    let gen_6 = step(&gen_5);
    //println!("{:#?}", env_xyz(&Point { x: 0, y: 0, z: 0 }))j

    gen_6.len()
}

fn main() {
    //let input = read_input("input_example.txt");
    let input = read_input("input.txt");
    let example_input = read_input("input_example.txt");
    println!("{}", input);

    println!("--- part I ------------------------------------------");
    let example_answer = part_1(&example_input);
    assert_eq!(example_answer, 112);

    let answer = part_1(&input);
    println!("{} live cells in 6th generation", answer);
    assert_eq!(answer, 313);

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
