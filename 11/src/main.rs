use std::fmt;
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

#[derive(Clone, Copy)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Floor => ".",
            Tile::EmptySeat => "L",
            Tile::OccupiedSeat => "#",
        };
        f.write_str(c)
    }
}

struct State {
    layout: Vec<Vec<Tile>>,
}

impl State {
    fn get_seat(&self, row_index: usize, seat_index: usize) -> &Tile {
        self.layout
            .get(row_index)
            .and_then(|row| row.get(seat_index))
            .unwrap_or(&Tile::Floor)
    }

    fn neighbors(&self, row_index: usize, seat_index: usize) -> Vec<&Tile> {
        let mut v = Vec::with_capacity(8);
        if row_index >= 1 {
            if seat_index >= 1 {
                v.push(self.get_seat(row_index - 1, seat_index - 1));
            }
            v.push(self.get_seat(row_index - 1, seat_index));
            v.push(self.get_seat(row_index - 1, seat_index + 1));
        }

        if seat_index >= 1 {
            v.push(self.get_seat(row_index, seat_index - 1));
            v.push(self.get_seat(row_index + 1, seat_index - 1));
        }

        v.push(self.get_seat(row_index, seat_index + 1));

        v.push(self.get_seat(row_index + 1, seat_index));
        v.push(self.get_seat(row_index + 1, seat_index + 1));

        v
    }
}

fn step(state: State) -> State {
    let mut rows = Vec::new();
    for (row_index, row) in state.layout.iter().enumerate() {
        let mut seats = Vec::new();
        for (seat_index, seat) in row.iter().enumerate() {
            let neighbors = state.neighbors(row_index, seat_index);

            let next = match seat {
                Tile::EmptySeat => {
                    if neighbors.iter().all(|s| match s {
                        Tile::OccupiedSeat => false,
                        _ => true,
                    }) {
                        Tile::OccupiedSeat
                    } else {
                        *seat
                    }
                }
                Tile::OccupiedSeat => {
                    let occupied = neighbors
                        .iter()
                        .filter(|s| match s {
                            Tile::OccupiedSeat => true,
                            _ => false,
                        })
                        .count();
                    if occupied >= 4 {
                        Tile::EmptySeat
                    } else {
                        Tile::OccupiedSeat
                    }
                }
                other => *other,
            };

            seats.push(next);
        }
        rows.push(seats);
    }

    State { layout: rows }
}

fn main() {
    //let input = read_input("input.txt");
    let input = read_input("input_example_1.txt");
    let mut state = State {
        layout: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Floor,
                        'L' => Tile::EmptySeat,
                        '#' => Tile::OccupiedSeat,
                        _ => panic!("invalid input!"),
                    })
                    .collect()
            })
            .collect(),
    };

    state = step(state);

    println!("{:?}", state.layout);

    println!("part I");
    println!("TODO");

    println!("part II");
    println!("TODO");
}
