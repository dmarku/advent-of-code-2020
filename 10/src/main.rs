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

fn main() {
    let _example_1 = "16
10
15
5
1
11
7
19
6
12
4";

    let _example_2 = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    //let input = _example_1;
    //let input = _example_2;
    let input = read_input("input.txt");

    println!("lines: {}", input.lines().count());
    let mut joltages: Vec<i32> = input.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    joltages.sort();

    println!("part I");

    joltages.insert(0, 0);
    joltages.push(joltages.last().unwrap() + 3);

    let differences: Vec<i32> = joltages
        .iter()
        .skip(1)
        .zip(joltages.iter())
        .map(|(a, b)| a - b)
        .collect();

    let mut bins: HashMap<&i32, u16> = HashMap::new();
    for d in &differences {
        let count = bins.entry(d).or_insert(0);
        *count += 1;
    }

    println!(
        "the product of 1-jolt differences and 3-jolt differences is {:?}",
        *bins.get(&1).unwrap_or(&0) * bins.get(&3).unwrap_or(&0)
    );

    println!("part II");

    // approach 2: "dynamic programming"
    // cache existing calculations
    let mut path_counts: HashMap<&i32, usize> = HashMap::new();
    path_counts.insert(joltages.last().unwrap(), 1);
    for j in joltages.iter().rev().skip(1) {
        path_counts.insert(
            j,
            (j + 1..=j + 3)
                .map(|nj| *path_counts.get(&nj).unwrap_or(&0))
                .sum(),
        );
    }

    println!(
        "{} possible adapter combinations",
        path_counts.get(joltages.first().unwrap()).unwrap()
    );
}
