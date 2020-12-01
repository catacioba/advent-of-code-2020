use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

fn convert_lines_to_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i64> {
    lines
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

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

fn main() {
    if let Ok(lines) = read_lines("src/ch01/input.txt") {
        let numbers = find_3_adding_numbers(convert_lines_to_numbers(lines));
        println!(
            "the numbers are {} and {} and {}",
            numbers.0, numbers.1, numbers.2
        );

        println!("their product is {}", numbers.0 * numbers.1 * numbers.2);
    }
    println!("done");
}
