use std::collections::{HashSet, HashMap};
use crate::utils::utils::{read_lines, convert_lines_to_numbers};

fn find_2_adding_numbers(numbers: Vec<i64>) -> (i64, i64) {
    let mut s: HashSet<i64> = HashSet::new();

    for num in numbers {
        // look for the number's complement in the set;
        let c = 2020 - num;

        if !s.contains(&c) {
            s.insert(num);
        } else {
            return (num, c);
        }
    }

    panic!("no 2 numbers that add up to 2020 found!")
}

pub fn solve_part1() {
    if let Ok(lines) = read_lines("src/ch01/input.txt") {
        let numbers = find_3_adding_numbers(convert_lines_to_numbers(lines));
        println!(
            "the numbers are {} and {} and {}",
            numbers.0, numbers.1, numbers.2
        );

        println!("their product is {}", numbers.0 * numbers.1 * numbers.2);
    }
    println!("part 1 done");
}

fn find_3_adding_numbers(numbers: Vec<i64>) -> (i64, i64, i64) {
    let mut m: HashMap<i64, i32> = HashMap::new();

    for num in numbers {
        m.entry(num).and_modify(|v| *v += 1).or_insert(1);
    }

    for (num1, c1) in &m {
        for num2 in m.keys() {
            if num1 == num2 && *c1 == 1 {
                continue;
            }

            let c = 2020 - num1 - num2;

            if let Some(c3) = m.get(&c) {
                if c == *num1 && *c3 == 2 {
                    continue;
                }
                return (*num1, *num2, c);
            }
        }
    }

    panic!("no 3 numbers thad add up to 2020 found!")
}

pub fn solve_part2() {
    if let Ok(lines) = read_lines("src/ch01/input.txt") {
        let numbers = find_2_adding_numbers(convert_lines_to_numbers(lines));
        println!(
            "the numbers are {} and {}",
            numbers.0, numbers.1
        );

        println!("their product is {}", numbers.0 * numbers.1);
    }
    println!("part 2 done");
}
