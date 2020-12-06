use crate::utils::utils::{read_lines, read_lines_until_empty};
use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::path;

fn group_answers_anyone(group: &str) -> usize {
    group
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<HashSet<char>>()
        .len()
}

pub fn solve_part1() {
    let answers: usize = read_lines_until_empty("src/ch06/input.txt")
        .iter()
        .map(|g| group_answers_anyone(g))
        .sum();

    println!("answers anyone: {}", answers);
}

fn group_answers_everyone(group: &str) -> usize {
    let answers: Vec<HashSet<char>> = group
        .lines()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .collect();

    let smallest_set = answers
        .iter()
        .min_by(|&s1, &s2| s1.len().cmp(&s2.len()))
        .unwrap();

    smallest_set
        .iter()
        .filter(|c| answers.iter().all(|s| s.contains(c)))
        .count()
}

pub fn solve_part2() {
    let answers: usize = read_lines_until_empty("src/ch06/input.txt")
        .iter()
        .map(|g| group_answers_everyone(g))
        .sum();

    println!("answers everyone: {}", answers);
}
