use std::f32::consts;
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

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn turn(&self, degrees: f32) -> Vector {
        let radians = degrees / 180.0 * consts::PI;
        Vector {
            x: radians.cos() * self.x - radians.sin() * self.y,
            y: radians.sin() * self.x + radians.cos() * self.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ship {
    x: f32,
    y: f32,
    direction: Direction,
    waypoint: Vector,
}

fn navigate(ship: &Ship, instruction: &str) -> Ship {
    let code = &instruction[..1];
    let distance = instruction[1..].parse::<f32>().unwrap();

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
            y: ship.y - distance,
            ..*ship
        },
        "S" => Ship {
            y: ship.y + distance,
            ..*ship
        },
        "E" => Ship {
            x: ship.x + distance,
            ..*ship
        },
        "W" => Ship {
            x: ship.x - distance,
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
    fn turn(&self, mut distance: f32) -> Direction {
        while distance < 0.0 {
            distance += 360.0
        }
        while distance >= 360.0 {
            distance -= 360.0
        }

        match distance {
            d if d == 0.0 => *self,
            d if d == 90.0 => match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            d if d == 180.0 => match self {
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
            },
            d if d == 270.0 => self.turn(90.0).turn(180.0),
            _ => *self,
        }
    }
}

fn navigate_pt2(ship: &Ship, instruction: &str) -> Ship {
    let code = &instruction[..1];
    let distance = instruction[1..].parse::<f32>().unwrap();

    match code {
        "F" => Ship {
            x: ship.x + ship.waypoint.x * distance,
            y: ship.y + ship.waypoint.y * distance,
            ..*ship
        },
        "N" => Ship {
            waypoint: Vector {
                y: ship.waypoint.y - distance,
                ..ship.waypoint
            },
            ..*ship
        },
        "S" => Ship {
            waypoint: Vector {
                y: ship.waypoint.y + distance,
                ..ship.waypoint
            },
            ..*ship
        },
        "E" => Ship {
            waypoint: Vector {
                x: ship.waypoint.x + distance,
                ..ship.waypoint
            },
            ..*ship
        },
        "W" => Ship {
            waypoint: Vector {
                x: ship.waypoint.x - distance,
                ..ship.waypoint
            },
            ..*ship
        },
        "L" => Ship {
            waypoint: ship.waypoint.turn(distance),
            ..*ship
        },
        "R" => Ship {
            waypoint: ship.waypoint.turn(-distance),
            ..*ship
        },
        _ => panic!("unknown instruction: {}", instruction),
    }
}

fn main() {
    let input = read_input("input.txt");
    print!("{}", input);

    let ship = Ship {
        x: 0.0,
        y: 0.0,
        direction: Direction::East,
        waypoint: Vector { x: 10.0, y: -1.0 },
    };
    println!("{:?}", ship);

    println!("--- part I ------------------------------------------");
    let final_ship = input
        .lines()
        .fold(ship, |ship, instruction| navigate(&ship, instruction));

    println!("{:?}", final_ship);
    println!(
        "Manhattan distance: {}",
        final_ship.x.abs() + final_ship.y.abs()
    );

    println!("--- part II -----------------------------------------");
    let final_ship = input
        .lines()
        .fold(ship, |ship, instruction| navigate_pt2(&ship, instruction));

    println!("{:?}", final_ship);
    println!(
        "Manhattan distance: {}",
        final_ship.x.abs() + final_ship.y.abs()
    );
}
