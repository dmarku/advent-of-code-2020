use std::convert::TryFrom;
use std::fs::File;
use std::hash::Hash;
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
struct Cell3D {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_cells_3d(s: &str) -> Option<Vec<Cell3D>> {
    let mut cells = Vec::new();
    for (y, l) in s.lines().enumerate() {
        let y = i32::try_from(y).ok()?;
        for (x, c) in l.chars().enumerate() {
            let x = i32::try_from(x).ok()?;
            if c == '#' {
                cells.push(Cell3D { x, y, z: 0 });
            }
        }
    }

    Some(cells)
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cell4D {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

fn parse_cells_4d(s: &str) -> Option<Vec<Cell4D>> {
    let mut cells = Vec::new();
    for (y, l) in s.lines().enumerate() {
        let y = i32::try_from(y).ok()?;
        for (x, c) in l.chars().enumerate() {
            let x = i32::try_from(x).ok()?;
            if c == '#' {
                cells.push(Cell4D { w: 0, x, y, z: 0 });
            }
        }
    }

    Some(cells)
}

enum Liveliness {
    Alive,
    Dead,
}

fn liveliness<P: Hash + Eq>(cells: &HashSet<P>, cell: &P) -> Liveliness {
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

fn step<C: Hash + Eq>(cells: &HashSet<C>, env: &dyn Fn(&C) -> IntoIter<C>) -> HashSet<C> {
    cells
        .iter()
        .flat_map(|c| env(c))
        .fold(HashMap::new(), |mut liveliness, cell| {
            *liveliness.entry(cell).or_insert(0) += 1;
            liveliness
        })
        .into_iter()
        .filter(
            |(p, neighbors)| match next_state(liveliness(cells, p), *neighbors) {
                Liveliness::Alive => true,
                Liveliness::Dead => false,
            },
        )
        .map(|(p, _)| p)
        .collect::<HashSet<C>>()
}

fn part_1(input: &str) -> usize {
    let temp_cells = parse_cells_3d(&input).unwrap();
    let cells: HashSet<Cell3D> = temp_cells.into_iter().collect();

    let gen_1 = step::<Cell3D>(&cells, &env3);
    let gen_2 = step::<Cell3D>(&gen_1, &env3);
    let gen_3 = step::<Cell3D>(&gen_2, &env3);
    let gen_4 = step::<Cell3D>(&gen_3, &env3);
    let gen_5 = step::<Cell3D>(&gen_4, &env3);
    let gen_6 = step::<Cell3D>(&gen_5, &env3);
    //println!("{:#?}", env_xyz(&Point { x: 0, y: 0, z: 0 }))j

    gen_6.len()
}

fn part_2(input: &str) -> usize {
    let temp_cells = parse_cells_4d(input).unwrap();
    let cells: HashSet<Cell4D> = temp_cells.into_iter().collect();

    let gen_1 = step::<Cell4D>(&cells, &env4);
    let gen_2 = step::<Cell4D>(&gen_1, &env4);
    let gen_3 = step::<Cell4D>(&gen_2, &env4);
    let gen_4 = step::<Cell4D>(&gen_3, &env4);
    let gen_5 = step::<Cell4D>(&gen_4, &env4);
    let gen_6 = step::<Cell4D>(&gen_5, &env4);
    //println!("{:#?}", env_xyz(&Point { x: 0, y: 0, z: 0 }))j

    gen_6.len()
}

fn env3(p: &Cell3D) -> IntoIter<Cell3D> {
    (-1..=1)
        .flat_map(move |x: i32| {
            (-1..=1).flat_map(move |y: i32| (-1..=1).map(move |z: i32| (x, y, z)))
        })
        .filter(|(x, y, z)| {
            vec![x.abs(), y.abs(), z.abs()]
                .into_iter()
                .max()
                .unwrap_or(0)
                == 1
        })
        .map(move |(x, y, z)| Cell3D {
            x: p.x + x,
            y: p.y + y,
            z: p.z + z,
        })
        .collect::<Vec<_>>()
        .into_iter()
}

fn env4(c: &Cell4D) -> IntoIter<Cell4D> {
    (-1..=1)
        .flat_map(move |w: i32| {
            (-1..=1).flat_map(move |x: i32| {
                (-1..=1).flat_map(move |y: i32| (-1..=1).map(move |z: i32| (w, x, y, z)))
            })
        })
        .filter(|(w, x, y, z)| {
            vec![w.abs(), x.abs(), y.abs(), z.abs()]
                .into_iter()
                .max()
                .unwrap_or(0)
                == 1
        })
        .map(move |(w, x, y, z)| Cell4D {
            w: c.w + w,
            x: c.x + x,
            y: c.y + y,
            z: c.z + z,
        })
        .collect::<Vec<_>>()
        .into_iter()
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
    println!("{} live 3D cells in 6th generation", answer);
    assert_eq!(answer, 313);

    println!("--- part II -----------------------------------------");
    let example_answer = part_2(&example_input);
    assert_eq!(example_answer, 848);

    let answer = part_2(&input);
    println!("{} live 4D cells in 6th generation", answer);
    assert_eq!(answer, 2640);
}
