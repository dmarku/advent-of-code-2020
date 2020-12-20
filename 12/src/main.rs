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

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

fn navigate(ship: &Ship, instruction: &str) -> Ship {
    let code = &instruction[..1];
    let distance = instruction[1..].parse::<i32>().unwrap();

    match code {
        "F" => match ship.direction {
            Direction::North => Ship {
                y: ship.y - distance,
                ..*ship
            },
            Direction::South => Ship {
                y: ship.y + distance,
                ..*ship
            },
            Direction::East => Ship {
                x: ship.x + distance,
                ..*ship
            },
            Direction::West => Ship {
                x: ship.x - distance,
                ..*ship
            },
        },
        "N" => Ship {
            x: ship.x - distance,
            ..*ship
        },
        "S" => Ship {
            x: ship.x + distance,
            ..*ship
        },
        "E" => Ship {
            y: ship.y + distance,
            ..*ship
        },
        "W" => Ship {
            y: ship.y - distance,
            ..*ship
        },
        "L" => Ship {
            direction: ship.direction.turn(distance),
            ..*ship
        },
        "R" => Ship {
            direction: ship.direction.turn(-distance),
            ..*ship
        },
        _ => panic!("unknown instruction: {}", instruction),
    }
}

impl Direction {
    /// turn counterclockwise
    fn turn(&self, mut distance: i32) -> Direction {
        while distance < 0 {
            distance += 360
        }
        while distance >= 360 {
            distance -= 360
        }

        match distance {
            0 => *self,
            90 => match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            180 => match self {
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
            },
            270 => self.turn(90).turn(180),
            _ => *self,
        }
    }
}

fn main() {
    let input = read_input("input.txt");
    print!("{}", input);

    println!("--- part I ------------------------------------------");

    let ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };
    println!("{:?}", ship);
    let final_ship = input
        .lines()
        .fold(ship, |ship, instruction| navigate(&ship, instruction));

    println!("{:?}", final_ship);
    println!(
        "Manhattan distance: {}",
        final_ship.x.abs() + final_ship.y.abs()
    );

    println!("--- part II -----------------------------------------");
    println!("TODO");
}
