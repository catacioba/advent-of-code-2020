use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::iter::FromIterator;
use std::str::Lines;

fn parse_ticket_row(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == 'F' {
                '0'
            } else if c == 'B' {
                '1'
            } else {
                panic!("Invalid row character: {}", c);
            }
        })
        .collect()
}

fn parse_ticket_col(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == 'R' {
                '1'
            } else if c == 'L' {
                '0'
            } else {
                panic!("Invalid column character: {}", c);
            }
        })
        .collect()
}

fn convert_binary_string_to_number(binary_str: String) -> i64 {
    let mut pow = 1;
    let mut accumm = 0;

    binary_str.chars().rev().for_each(|c| {
        if c == '1' {
            accumm += pow;
        }
        pow = pow * 2;
    });

    accumm
}

fn parse_ticket(s: &str) -> i64 {
    let (row, col) = s.split_at(7);

    let row_value = convert_binary_string_to_number(parse_ticket_row(row));
    let col_value = convert_binary_string_to_number(parse_ticket_col(col));
    let seat_value = (row_value * 8) + col_value;

    println!(
        "row: {} parsed_row: {} col: {} parsed_column: {} seat_value: {}",
        row, row_value, col, col_value, seat_value
    );

    seat_value
}

pub fn solve_part1() {
    let best_seat: i64 = fs::read_to_string("src/ch05/input.txt")
        .unwrap()
        .lines()
        .map(|line| parse_ticket(line))
        .max()
        .unwrap();

    println!("best seat: {}", best_seat);
}

fn find_missing_seat(seat_ids: Vec<i64>) -> i64 {
    let seat_ids_set: HashSet<i64> = HashSet::from_iter(seat_ids.clone());

    let min_seat_id = *seat_ids.iter().min().unwrap();
    let max_seat_id = *seat_ids.iter().max().unwrap();

    for seat_id in min_seat_id..max_seat_id {
        if !seat_ids_set.contains(&seat_id)
            && seat_ids_set.contains(&(seat_id - 1))
            && seat_ids_set.contains(&(seat_id + 1))
        {
            return seat_id;
        }
    }
    panic!("Seat id not found!");
}

pub fn solve_part2() {
    let seat_ids: Vec<i64> = fs::read_to_string("src/ch05/input.txt")
        .unwrap()
        .lines()
        .map(|line| parse_ticket(line))
        .collect();

    println!("missing seat id: {}", find_missing_seat(seat_ids));
}
