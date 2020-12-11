use std::collections::HashMap;
use std::fs;

pub fn solve_part1() {
    let mut numbers: Vec<u64> = fs::read_to_string("src/ch10/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    numbers.sort();

    let mut plus_ones = 0;
    let mut plus_threes = 0;
    let mut current = 0;

    for number in &numbers {
        if *number > current + 3 {
            panic!("Could not reach {} from {}", number, current);
        }
        let diff = *number - current;

        if diff == 1 {
            plus_ones += 1;
        } else if diff == 3 {
            plus_threes += 1;
        }

        current = *number;
    }

    println!("{}", plus_ones * (plus_threes + 1));
}

fn get_possibilities(possibilities: &HashMap<u64, u64>, number: u64) -> u64 {
    let mut accumulator = 0;

    if number >= 1 {
        accumulator += possibilities.get(&(number - 1)).unwrap_or(&0);
    }
    if number >= 2 {
        accumulator += possibilities.get(&(number - 2)).unwrap_or(&0);
    }
    if number >= 3 {
        accumulator += possibilities.get(&(number - 3)).unwrap_or(&0);
    }

    accumulator
}

pub fn solve_part2() {
    let mut numbers: Vec<u64> = fs::read_to_string("src/ch10/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    numbers.sort();

    let mut possibilities: HashMap<u64, u64> = HashMap::new();
    possibilities.insert(0, 1);

    numbers.iter().for_each(|num| {
        let accum = get_possibilities(&possibilities, *num);
        possibilities.insert(*num, accum);
    });

    let final_possibilities = get_possibilities(&possibilities, *numbers.last().unwrap());

    println!("possibilities: {}", final_possibilities);
}
