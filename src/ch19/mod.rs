use crate::utils::utils::read_lines_until_empty;
use std::collections::HashMap;

type Pattern = Vec<usize>;

#[derive(Debug, PartialOrd, PartialEq)]
enum Rule {
    Character(char),
    Sequence(Vec<Pattern>),
}

fn parse_rule(rule: &str) -> (usize, Rule) {
    let mut parts = rule.split(":");
    let rule_number = parts.next().unwrap().parse::<usize>().unwrap();
    let rule_part = parts.next().unwrap();

    let rule = if rule_part.contains("\"") {
        if rule_part.contains("a") {
            Rule::Character('a')
        } else {
            Rule::Character('b')
        }
    } else {
        Rule::Sequence(
            rule_part
                .split("|")
                .map(|p| {
                    p.split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect(),
        )
    };

    (rule_number, rule)
}

fn parse_rules(rules: &str) -> HashMap<usize, Rule> {
    rules.lines().map(|l| parse_rule(l)).collect()
}

fn match_char(s: &str, c: char) -> bool {
    s.chars().next() == Some(c)
}

fn match_seq(s: &str, seq: &Pattern, rules: &HashMap<usize, Rule>) -> Option<usize> {
    let mut current = 0;

    for r in seq {
        if current >= s.len() {
            return None;
        }

        let r = rules.get(r).unwrap();
        let option = match_rule(&s[current..], r, rules);

        if let Some(match_index) = option {
            current += match_index
        } else {
            return None;
        }
    }
    Some(current)
}

fn match_rule(s: &str, r: &Rule, rules: &HashMap<usize, Rule>) -> Option<usize> {
    match r {
        Rule::Character(c) => return if !match_char(s, *c) { None } else { Some(1) },
        Rule::Sequence(seq) => {
            for p in seq {
                let m = match_seq(s, p, rules);

                if m.is_some() {
                    return m;
                }
            }
        }
    }

    None
}

fn matched_full(s: &str, r: &Rule, rules: &HashMap<usize, Rule>) -> bool {
    let maybe_matched = match_rule(s, r, rules);

    if let Some(last_index) = maybe_matched {
        last_index == s.len()
    } else {
        false
    }
}

pub fn solve_part1() {
    let contents = read_lines_until_empty("src/ch19/input.txt");
    let rules = &contents[0];

    let rules = parse_rules(rules);
    let rule = rules.get(&0).unwrap();

    let matching_lines = &contents[1]
        .lines()
        .filter(|l| matched_full(l, rule, &rules))
        .count();

    println!("Matching rules: {}", matching_lines);
}

fn compute_possible_rules() -> Rule {
    // we need to match rule 42 at least 2 times followed by at least 1 31
    let mut possible_sequences = Vec::new();

    for x in 2..200 {
        for y in 1..x {
            // println!("{} {}", x, y);
            let mut acc = Vec::new();
            for _ in 0..x {
                acc.push(42);
            }
            for _ in 0..y {
                acc.push(31);
            }
            possible_sequences.push(acc);
        }
    }

    Rule::Sequence(possible_sequences)
}

fn match_rec(s: &str, rules: &HashMap<usize, Rule>) -> bool {
    let mut current = 0;

    let mut count_42 = 0;
    loop {
        let s = &s[current..];

        if let Some(match_index) = match_seq(s, &vec![42], rules) {
            count_42 += 1;
            current += match_index
        } else {
            break;
        }
    }

    let mut count_31 = 0;
    loop {
        let s = &s[current..];

        if let Some(match_index) = match_seq(s, &vec![31], rules) {
            count_31 += 1;
            current += match_index
        } else {
            break;
        }
    }

    count_42 >= 2 && count_42 > count_31 && count_31 >= 1 && current == s.len()
}

pub fn solve_part2() {
    let contents = read_lines_until_empty("src/ch19/input.txt");
    let rules = &contents[0];

    let mut rules = parse_rules(rules);
    rules.remove(&8);
    rules.remove(&11);

    let matching_lines: Vec<&str> = contents[1]
        .lines()
        .filter(|l| match_rec(l, &rules))
        .collect();

    for l in &matching_lines {
        println!("{:?}", l);
    }

    println!("Matching rules: {}", matching_lines.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_simple_rules() {
        let rules = parse_rules("0: 1 2\r\n1: \"a\"\r\n2: 1 3 | 3 1\r\n3: \"b\"");

        assert_eq!(rules.get(&0), Some(&Rule::Sequence(vec![vec![1, 2]])));
        assert_eq!(rules.get(&1), Some(&Rule::Character('a')));
        assert_eq!(
            rules.get(&2),
            Some(&Rule::Sequence(vec![vec![1, 3], vec![3, 1]]))
        );
        assert_eq!(rules.get(&3), Some(&Rule::Character('b')));
    }

    #[test]
    fn test_parse_rule_complex_rules() {
        let rules = parse_rules(
            "0: 4 1 5\r\n1: 2 3 | 3 2\r\n2: 4 4 | 5 5\r\n3: 4 5 | 5 4\r\n4: \"a\"\r\n5: \"b\"",
        );

        assert_eq!(rules.get(&0), Some(&Rule::Sequence(vec![vec![4, 1, 5]])));
        assert_eq!(
            rules.get(&1),
            Some(&Rule::Sequence(vec![vec![2, 3], vec![3, 2]]))
        );
        assert_eq!(
            rules.get(&2),
            Some(&Rule::Sequence(vec![vec![4, 4], vec![5, 5]]))
        );
        assert_eq!(
            rules.get(&3),
            Some(&Rule::Sequence(vec![vec![4, 5], vec![5, 4]]))
        );
        assert_eq!(rules.get(&4), Some(&Rule::Character('a')));
        assert_eq!(rules.get(&5), Some(&Rule::Character('b')));
    }

    #[test]
    fn test_match_char() {
        assert_eq!(match_char("", 'a'), false);
        assert_eq!(match_char("a", 'a'), true);
        assert_eq!(match_char("abc", 'a'), true);
        assert_eq!(match_char("cba", 'a'), false);
    }

    #[test]
    fn test_match_seq_only_chars() {
        let rules = parse_rules("4: \"a\"\r\n5: \"b\"");

        assert_eq!(match_seq("ab", &vec![4, 5], &rules), Some(2));
        assert_eq!(match_seq("ba", &vec![5, 4], &rules), Some(2));
        assert_eq!(match_seq("", &vec![5, 4], &rules), None);
        assert_eq!(match_seq("ab", &vec![5, 4], &rules), None);
        assert_eq!(match_seq("ba", &vec![5, 4, 4], &rules), None);
        assert_eq!(match_seq("aba", &vec![4, 5, 4], &rules), Some(3));
    }

    #[test]
    fn test_match_rule_complex() {
        let rules = parse_rules("4: \"a\"\r\n5: \"b\"");

        assert_eq!(match_rule("ab", &Rule::Character('a'), &rules), Some(1));
        assert_eq!(match_rule("ab", &Rule::Character('b'), &rules), None);
        assert_eq!(
            match_rule("ab", &Rule::Sequence(vec![vec![4, 4]]), &rules),
            None
        );
        assert_eq!(
            match_rule("ab", &Rule::Sequence(vec![vec![4, 5]]), &rules),
            Some(2)
        );
        assert_eq!(
            match_rule("ab", &Rule::Sequence(vec![vec![4, 4], vec![4, 5]]), &rules),
            Some(2)
        );
    }
}
