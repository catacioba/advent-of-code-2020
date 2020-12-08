use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_rhs_bag(rhs_bag: &str) -> (i32, String) {
    let first_name_pos = rhs_bag
        .chars()
        .position(|c| c.is_ascii_lowercase())
        .unwrap();
    let final_name_pos = if rhs_bag.ends_with("s") {
        rhs_bag.len() - 5
    } else {
        rhs_bag.len() - 4
    };

    (
        rhs_bag[..first_name_pos - 1].parse().unwrap(),
        rhs_bag[first_name_pos..final_name_pos].to_string(),
    )
}

fn parse_rhs(rhs: &str) -> Vec<(i32, String)> {
    rhs.split(", ").map(|b| parse_rhs_bag((b))).collect()
}

fn parse_line(mut line: String) -> (String, Vec<(i32, String)>) {
    if line.ends_with(".") {
        line.pop();
    }

    let mut s = line.split(" bags contain ");
    let lhs = s.next().unwrap().to_string();
    let rhs = s.next().unwrap();

    let right_bags = if rhs != "no other bags" {
        parse_rhs(rhs)
    } else {
        Vec::new()
    };

    (lhs, right_bags)
}

fn parse_rules() -> HashMap<String, Vec<(i32, String)>> {
    fs::read_to_string("src/ch07/input.txt")
        .unwrap()
        .lines()
        .map(|l| parse_line(l.to_string()))
        .collect()
}

// fn dfs(current: &Rule, target: &str, rules: HashMap<String, Vec<String>>, visited: &mut HashMap<&String, bool>) {
//     // rules
//     //     .iter()
//     //     .map(|r| {
//     //         r.to.iter()
//     //             .map(|to| (&r.from, to))
//     //             .collect::<Vec<(&String, &String)>>()
//     //     })
//     //     .flatten()
//     //     .collect()
//     for bag in current.
// }

fn dfs_dumb(current: &str, target: &str, rules: &HashMap<String, Vec<(i32, String)>>) -> bool {
    if current == target {
        return true;
    }

    rules
        .get(current)
        .unwrap()
        .iter()
        .any(|r: &(i32, String)| dfs_dumb(&r.1, target, rules))
}

pub fn solve_part1() {
    let rules = parse_rules();

    let target = "shiny gold";

    let count = rules
        .keys()
        .filter(|r| *r != target && dfs_dumb(r, target, &rules))
        .count();

    println!("count: {}", count);
}

fn count_bags(current: String, rules: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    rules
        .get(&current)
        .unwrap()
        .iter()
        .map(|(c, b)| c * (1 + count_bags(String::from(b), rules)))
        .sum()
}

pub fn solve_part2() {
    let rules = parse_rules();

    let count = count_bags(String::from("shiny gold"), &rules);

    println!("count: {}", count);
}
