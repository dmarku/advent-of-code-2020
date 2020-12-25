use std::collections::HashMap;
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

struct Game {
    most_recent_turn: HashMap<usize, usize>,
    turn: usize,
    last: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            most_recent_turn: HashMap::new(),
            turn: 0,
            last: 0,
        }
    }

    fn add(&mut self, n: usize) {
        self.most_recent_turn.insert(self.last, self.turn);
        self.turn += 1;
        self.last = n;
    }

    fn play(&mut self) {
        let n = self
            .most_recent_turn
            .get(&self.last)
            .and_then(|n| Some(self.turn - n))
            .unwrap_or(0);
        self.add(n);
    }
}

fn main() {
    let input = read_input("input.txt");
    //println!("{}", input);

    println!("--- part I ------------------------------------------");

    let final_turn = 2020;
    let mut turns = Vec::with_capacity(final_turn);

    fn next_turn(turns: &[usize]) -> usize {
        if let Some((last, rest)) = turns.split_last() {
            // find the last turn this number was announced
            rest.iter()
                .rev()
                .enumerate()
                .find_map(|(i, n)| if n == last { Some(i + 1) } else { None })
                .unwrap_or(0)
        } else {
            0
        }
    }

    for n in input.split(",").map(|s| s.parse::<usize>()) {
        turns.push(n.unwrap());
    }

    for _ in turns.len()..final_turn {
        turns.push(next_turn(&turns));
    }

    //println!("{:?}", turns);
    println!(
        "the number on turn {} is {}",
        final_turn,
        turns.last().unwrap()
    );

    println!("--- part II -----------------------------------------");

    let final_turn = 30000000;
    let mut game = Game::new();

    for n in input.split(",").map(|s| s.parse::<usize>()) {
        game.add(n.unwrap());
    }

    for _ in game.turn..final_turn {
        game.play();
        //print!("\r{}", game.turn);
    }

    //println!("{:?}", turns);
    println!("the number on turn {} is {}", final_turn, game.last);
}
