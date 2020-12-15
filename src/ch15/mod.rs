use std::collections::HashMap;
use std::fs;

fn find_number(numbers: Vec<u64>, last_iteration: usize) -> u64 {
    let mut numbers_spoken: HashMap<u64, u64> = numbers
        .iter()
        .enumerate()
        .map(|(idx, n)| (*n, idx as u64))
        .collect();
    let mut last_spoken_number = numbers[numbers.len() - 1];
    numbers_spoken.remove(&last_spoken_number);

    let step = last_iteration / 10;
    for iteration in numbers.len()..last_iteration {
        if iteration % step == 0 {
            println!("{}", iteration);
        }

        let previous_iteration = iteration as u64 - 1;
        let previous_number_spoken = last_spoken_number;

        if numbers_spoken.contains_key(&last_spoken_number) {
            let diff = previous_iteration - numbers_spoken.get(&last_spoken_number).unwrap();
            last_spoken_number = diff;
        } else {
            last_spoken_number = 0;
        }

        numbers_spoken.insert(previous_number_spoken, previous_iteration);
    }

    last_spoken_number
}

pub fn solve_part1() {
    println!("{}", find_number(vec![0, 13, 1, 16, 6, 17], 2020));
}

pub fn solve_part2() {
    println!("{}", find_number(vec![0, 13, 1, 16, 6, 17], 30000000));
}

#[cfg(test)]
mod tests {
    use crate::ch15::find_number;

    #[test]
    fn find_number_works() {
        let last_iteration = 2020;
        assert_eq!(find_number(vec![0, 3, 6], last_iteration), 436);
        assert_eq!(find_number(vec![1, 3, 2], last_iteration), 1);
        assert_eq!(find_number(vec![2, 1, 3], last_iteration), 10);
        assert_eq!(find_number(vec![1, 2, 3], last_iteration), 27);
        assert_eq!(find_number(vec![2, 3, 1], last_iteration), 78);
        assert_eq!(find_number(vec![3, 2, 1], last_iteration), 438);
        assert_eq!(find_number(vec![3, 1, 2], last_iteration), 1836);
    }
}
