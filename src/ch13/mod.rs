use crate::utils::utils::read_lines;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn find_earliest_time(time: i64, buses: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut earliest_time = time;

    loop {
        if let Some((_, bus)) = buses.iter().find(|(_, b)| earliest_time % *b == 0) {
            return (*bus, earliest_time);
        }
        earliest_time += 1;
    }
}

fn read_input() -> (i64, Vec<(i64, i64)>) {
    let mut lines = read_lines("src/ch13/input.txt").unwrap();

    let time: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let buses: Vec<(i64, i64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(idx, s)| *s != "x")
        .map(|(idx, s)| (idx as i64, s.parse::<i64>().unwrap()))
        .collect();

    (time, buses)
}

pub fn solve_part1() {
    let (time, buses) = read_input();

    println!("time: {}", time);
    println!("buses: {:?}", buses);

    let (bus, earliest_time) = find_earliest_time(time, &buses);

    println!("earliest time: {}", bus * (earliest_time - time));
}

fn is_valid(num: i64, buses: &Vec<(i64, i64)>) -> bool {
    buses.iter().all(|&(idx, b)| (num + idx as i64) % b == 0)
}

fn find_time(buses: &Vec<(i64, i64)>) -> i64 {
    let &(idx, max_value) = buses.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
    let mut current_value = max_value - idx as i64;

    loop {
        println!("Trying: {}", current_value);

        if is_valid(current_value, buses) {
            return current_value;
        }

        current_value += max_value;
    }
}

fn euclidean(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    return euclidean(b, a % b);
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, x0, y0) = extended_euclidean(b, a % b);
    (d, y0, x0 - (a / b) * y0)
}

fn modular_inverse(a: i64, modulo: i64) -> i64 {
    let (d, x, _) = extended_euclidean(a, modulo);
    if d != 1 {
        panic!("no modular inverse");
    }
    x
}

fn solve_extended_euclidean(buses: &Vec<(i64, i64)>) -> i64 {
    let m: i64 = buses.iter().map(|&(_, b)| b as i64).product();

    let solution = buses
        .iter()
        .map(|&(idx, bus)| {
            // t + idx = 0 mod bus => t = (bus - idx) mod bus
            let a = -(idx as i64);
            let b = bus as i64;

            let b_i = m / b;
            let b_i_inverse = modular_inverse(b_i, b);

            a * b_i * b_i_inverse
        })
        .sum::<i64>();

    (solution + m) % m
}

pub fn solve_part2() {
    let (_, buses) = read_input();

    println!("{}", solve_extended_euclidean(&buses));
}
