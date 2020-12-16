use crate::utils::utils::read_lines;

#[derive(Debug)]
struct PasswordPolicy {
    character: char,
    lower_limit: i32,
    upper_limit: i32,
}

fn parse_policy(line: String) -> PasswordPolicy {
    let mut parts_iterator = line.split_whitespace();

    let mut limit_iterator = parts_iterator.next().unwrap().split("-");

    PasswordPolicy {
        character: parts_iterator.next().unwrap().chars().next().unwrap(),
        lower_limit: limit_iterator.next().unwrap().parse().unwrap(),
        upper_limit: limit_iterator.next().unwrap().parse().unwrap(),
    }
}

#[derive(Debug)]
struct Password {
    policy: PasswordPolicy,
    value: String,
}

fn parse_password(line: String) -> Password {
    let mut parts_iterator = line.split(":");

    Password {
        policy: parse_policy(parts_iterator.next().unwrap().to_string()),
        value: parts_iterator.next().unwrap()[1..].to_string(),
    }
}

const DELTA: usize = 'a' as usize;

fn char_to_index(c: char) -> usize {
    c as usize - DELTA
}

fn is_password_valid_part1(p: &Password) -> bool {
    let policy = &p.policy;

    let mut char_frequency: [i32; 26] = [0; 26];

    for c in p.value.chars() {
        char_frequency[char_to_index(c)] += 1
    }

    let freq = char_frequency[char_to_index(p.policy.character)];

    freq >= p.policy.lower_limit && freq <= p.policy.upper_limit
}

fn is_password_valid_part2(p: &Password) -> bool {
    let policy = &p.policy;

    let mut cnt = 0;

    for (idx, c) in p.value.char_indices() {
        if idx + 1 == policy.lower_limit as usize && c == p.policy.character {
            cnt += 1;
        } else if idx + 1 == policy.upper_limit as usize && c == p.policy.character {
            cnt += 1;
        } else if idx + 1 > policy.upper_limit as usize {
            break;
        }
    }

    cnt == 1
}

fn get_invalid_passwords_count_part1(passwords: Vec<Password>) -> usize {
    passwords
        .iter()
        .filter(|p| is_password_valid_part1(p))
        .count()
}

fn get_invalid_passwords_count_part2(passwords: Vec<Password>) -> usize {
    passwords
        .iter()
        .filter(|p| is_password_valid_part2(p))
        .count()
}

pub fn solve_part1() {
    if let Ok(lines) = read_lines("src/ch02/input.txt") {
        let passwords: Vec<Password> = lines.map(|l| parse_password(l.unwrap())).collect();
        println!("password file parsed");
        println!(
            "invalid password count part 1 => {}",
            get_invalid_passwords_count_part1(passwords)
        );
    }
}

pub fn solve_part2() {
    if let Ok(lines) = read_lines("src/ch02/input.txt") {
        let passwords: Vec<Password> = lines.map(|l| parse_password(l.unwrap())).collect();
        println!("password file parsed");
        println!(
            "invalid password count part 2 => {}",
            get_invalid_passwords_count_part2(passwords)
        );
    }
}
