use crate::utils::utils::read_lines;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

#[derive(Debug)]
struct Passport {
    values: HashMap<String, String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        if self.values.contains_key("cid") {
            self.values.len() == 8
        } else {
            self.values.len() == 7
        }
    }

    fn is_valid_part2(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        for (field, value) in &self.values {
            if !validate_field(&field, &value) {
                // println!("{} => {} invalid!!", field, value);
                return false;
            }
        }

        true
    }
}

fn read_passports() -> Vec<Passport> {
    if let Ok(lines) = read_lines("src/ch04/input.txt") {
        let mut passport = HashMap::new();
        let mut v = Vec::new();

        for line in lines {
            let l = line.unwrap();

            if l.is_empty() {
                v.push(Passport { values: passport });
                passport = HashMap::new();
            } else {
                l.split_whitespace().for_each(|p| {
                    let mut it = p.split(":");
                    let key: String = String::from(it.next().unwrap());
                    let value: String = String::from(it.next().unwrap());

                    passport.insert(key, value);
                });
            }
        }
        if !passport.is_empty() {
            v.push(Passport { values: passport });
        }

        v
    } else {
        panic!("File not found.");
    }
}

fn validate_number(value: &str, from: i32, to: i32) -> bool {
    match value.parse::<i32>() {
        Ok(num) => num >= from && num <= to,
        Err(_) => false,
    }
}

fn validate_eye_color(x: &str) -> bool {
    let mut s: HashSet<&str> = HashSet::new();
    s.insert("amb");
    s.insert("blu");
    s.insert("brn");
    s.insert("gry");
    s.insert("grn");
    s.insert("hzl");
    s.insert("oth");

    s.contains(&x)
}

fn validate_height(s: &str) -> bool {
    let n = s.len() - 2;
    if s.len() < 4 {
        return false;
    }

    if s.ends_with("cm") {
        validate_number(s.split_at(n).0, 150, 193)
    } else if s.ends_with("in") {
        validate_number(s.split_at(n).0, 59, 76)
    } else {
        false
    }
}

fn validate_hair_color(s: &str) -> bool {
    for (pos, ch) in s.chars().enumerate() {
        if pos == 0 {
            if ch != '#' {
                return false;
            }
        } else if pos <= 6 {
            if !ch.to_ascii_lowercase().is_ascii_alphanumeric() {
                return false;
            }
        } else if pos > 6 {
            return false;
        }
    }
    true
}

fn validate_pid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| c.is_numeric())
}

fn validate_cid(_s: &str) -> bool {
    true
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => validate_number(value, 1920, 2002),
        "iyr" => validate_number(value, 2010, 2020),
        "eyr" => validate_number(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair_color(value),
        "ecl" => validate_eye_color(value),
        "pid" => validate_pid(value),
        "cid" => validate_cid(value),
        _ => false,
    }
}

pub fn solve_part1() {
    let passports = read_passports();

    let valid_passports = passports.iter().filter(|p| p.is_valid()).count();

    println!("valid passports: {}", valid_passports);
}

pub fn solve_part2() {
    let passwords = read_passports();

    let valid_passports = passwords.iter().filter(|p| p.is_valid_part2()).count();

    println!("valid passports: {}", valid_passports);
}
