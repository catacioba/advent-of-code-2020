use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn is_valid(numbers: &Vec<u64>, current_position: usize, k: usize) -> bool {
    let num = numbers[current_position];

    for y in current_position - k..current_position {
        for z in y + 1..current_position {
            if numbers[y] + numbers[z] == num {
                return true;
            }
        }
    }

    false
}

fn find_first_invalid(numbers: &Vec<u64>, k: usize) -> u64 {
    for x in k..numbers.len() {
        if !is_valid(numbers, x, k) {
            return numbers[x];
        }
    }
    panic!("no invalid number found");
}

fn read_numbers() -> Vec<u64> {
    fs::read_to_string("src/ch09/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

pub fn solve_part1() {
    let numbers = read_numbers();
    let k = 25;

    let first_incorrect = find_first_invalid(&numbers, k);

    println!("first incorrect: {}", first_incorrect);
}

fn compute_partial_sums(numbers: &Vec<u64>) -> Vec<u64> {
    let mut accumulator = 0;
    numbers
        .iter()
        .cloned()
        .map(|x| {
            accumulator += x;
            accumulator.clone()
        })
        .collect()
}

fn find_weakness(numbers: &Vec<u64>, sum: u64) -> (usize, usize) {
    let mut start = 0;
    let mut end = 0;
    let mut accumulator = 0;

    while end < numbers.len() && start <= end {
        if accumulator == sum {
            return (start, end);
        } else if accumulator < sum {
            accumulator += numbers[end];
            end += 1;
        } else {
            accumulator -= numbers[start];
            start += 1;
        }
    }

    panic!("no weakness found");
}

pub fn solve_part2() {
    let numbers = read_numbers();
    // let partial_sums = compute_partial_sums(&numbers);

    let k = 25;
    let first_incorrect = find_first_invalid(&numbers, k);
    println!("first incorrect: {}", first_incorrect);

    let weakness_bounds = find_weakness(&numbers, first_incorrect);
    let weakness_range = &numbers[weakness_bounds.0..weakness_bounds.1];

    let smallest = weakness_range.iter().min().unwrap();
    let highest = weakness_range.iter().max().unwrap();

    let weakness = smallest + highest;

    println!("weakness: {}", weakness);
}
