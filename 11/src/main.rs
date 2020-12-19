use std::cmp;
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

#[derive(Clone, Copy, Eq)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl std::cmp::PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Tile::Floor, Tile::Floor) => true,
            (Tile::EmptySeat, Tile::EmptySeat) => true,
            (Tile::OccupiedSeat, Tile::OccupiedSeat) => true,
            _ => false,
        }
    }
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

#[derive(PartialEq)]
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

    fn get_neighbors(&self, row_index: usize, seat_index: usize) -> Vec<&Tile> {
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

    fn occupied_count(&self, row_index: usize, seat_index: usize) -> usize {
        let mut occupied_seats = 0;

        //north
        {
            let steps = row_index;
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index - step, seat_index) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //northeast
        {
            let steps = cmp::min(row_index, self.layout[0].len() - seat_index);
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index - step, seat_index + step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }
        //east
        {
            let steps = self.layout[0].len() - seat_index;
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index, seat_index + step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //southeast
        {
            let steps = cmp::min(
                self.layout.len() - row_index,
                self.layout[0].len() - seat_index,
            );
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index + step, seat_index + step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //south
        {
            let steps = self.layout.len() - row_index;
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index + step, seat_index) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //southwest
        {
            let steps = cmp::min(self.layout.len() - row_index, seat_index);
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index + step, seat_index - step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //west
        {
            let steps = seat_index;
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index, seat_index - step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        //northwest
        {
            let steps = cmp::min(row_index, seat_index);
            let mut occupied = false;
            for step in 1..steps {
                if let Tile::OccupiedSeat = self.get_seat(row_index - step, seat_index - step) {
                    occupied = true;
                }
            }
            if occupied {
                occupied_seats += 1;
            }
        }

        occupied_seats
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.layout {
            for seat in row {
                f.write_fmt(format_args!("{:?}", &seat))?;
            }
            f.write_str("\n")?;
        }

        fmt::Result::Ok(())
    }
}

fn step(state: &State) -> State {
    let mut rows = Vec::new();
    for (row_index, row) in state.layout.iter().enumerate() {
        let mut seats = Vec::new();
        for (seat_index, seat) in row.iter().enumerate() {
            let neighbors = state.get_neighbors(row_index, seat_index);

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

fn step_pt2(state: &State) -> State {
    let mut rows = Vec::new();
    for (row_index, row) in state.layout.iter().enumerate() {
        let mut seats = Vec::new();
        for (seat_index, seat) in row.iter().enumerate() {
            let occupied_seats = state.occupied_count(row_index, seat_index);

            let next = match seat {
                Tile::EmptySeat => {
                    if occupied_seats == 0 {
                        Tile::OccupiedSeat
                    } else {
                        *seat
                    }
                }
                Tile::OccupiedSeat => {
                    if occupied_seats >= 5 {
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
    let initial_state = State {
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

    println!("--- part I --------------------");

    {
        let mut state = State {
            layout: initial_state.layout.clone(),
        };
        let mut next = step(&state);
        let mut steps = 1;

        while state != next {
            steps += 1;
            state = next;
            next = step(&state);
        }

        println!("{} steps until equlibrium", steps);
        let occupied_seats: usize = state
            .layout
            .iter()
            .map(|row| row.iter().filter(|&s| *s == Tile::OccupiedSeat).count())
            .sum();
        println!("{} seats occupied", occupied_seats);
    }

    println!("--- part II --------------------");
    {
        let mut state = State {
            layout: initial_state.layout.clone(),
        };
        let mut next = step_pt2(&state);
        let mut steps = 1;

        while state != next {
            steps += 1;
            state = next;
            next = step_pt2(&state);
        }

        println!("{} steps until equlibrium", steps);
        let occupied_seats: usize = state
            .layout
            .iter()
            .map(|row| row.iter().filter(|&s| *s == Tile::OccupiedSeat).count())
            .sum();
        println!("{} seats occupied", occupied_seats);
    }

    let first_step = step_pt2(&initial_state);
    let second_step = step_pt2(&first_step);
    println!("{:?}", second_step);
}
