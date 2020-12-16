use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Rule {
    field: String,
    ranges: Vec<(u64, u64)>,
}

fn parse_range(range: &str) -> (u64, u64) {
    let mut it = range.split("-");
    (
        it.next()
            .expect(format!("Expected to have a range start in {}", range).as_str())
            .parse()
            .unwrap(),
        it.next()
            .expect(format!("Expected to have a range end in {}", range).as_str())
            .parse()
            .unwrap(),
    )
}

impl Rule {
    fn from_line(line: &str) -> Rule {
        let mut parts = line.split(": ");

        let field = parts.next().expect("Expected to have a rule field part!");
        let ranges = parts
            .next()
            .expect("Expected to have a rule range part!")
            .split(" or ")
            .map(|r| parse_range(r))
            .collect();

        Rule {
            field: String::from(field),
            ranges,
        }
    }

    fn contain(&self, value: &u64) -> bool {
        self.ranges.iter().any(|(l, r)| l <= value && value <= r)
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn from_line(line: &str) -> Ticket {
        Ticket {
            values: line
                .split(",")
                .map(|s| {
                    s.parse::<u64>()
                        .expect(format!("Could not parse {}", s).as_str())
                })
                .collect(),
        }
    }
}

fn read_input() -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let contents = fs::read_to_string("src/ch16/input.txt").unwrap();
    let mut it = contents.split("\r\n\r\n");

    let rules: Vec<Rule> = it
        .next()
        .unwrap()
        .lines()
        .map(|r| Rule::from_line(r))
        .collect();

    let ticket = Ticket::from_line(it.next().unwrap().lines().skip(1).next().unwrap());

    let nearby_tickets: Vec<Ticket> = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| Ticket::from_line(l))
        .collect();

    (rules, ticket, nearby_tickets)
}

fn is_valid(value: u64, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|r| r.contain(&value))
}

pub fn solve_part1() {
    let (rules, _, nearby_tickets) = read_input();

    let ticket_scanning_error_rate: u64 = nearby_tickets
        .iter()
        .map(|t| -> u64 {
            t.values
                .iter()
                .map(|v| if is_valid(*v, &rules) { 0 } else { *v })
                .sum()
        })
        .sum();

    println!("ticket error rate: {}", ticket_scanning_error_rate);
}

fn find_field_order<'a>(tickets: &Vec<&Ticket>, rules: &'a Vec<Rule>) -> HashMap<&'a str, usize> {
    /*
    first   ->          row
    second  ->  class + row
    third   ->  class + row + seat

    row     -> 1 + 2 + 3
    class   ->     2 + 3
    seat    ->         3
     */
    let mut field_columns: HashMap<&str, HashSet<usize>> = HashMap::new();

    let mut fields_in_degree = HashMap::new();
    let column_count = rules.len();

    for rule in rules {
        let mut possible_columns = HashSet::new();

        for column in 0..column_count {
            if tickets.iter().all(|t| rule.contain(&t.values[column])) {
                possible_columns.insert(column);
                fields_in_degree
                    .entry(&rule.field)
                    .and_modify(|d| *d += 1)
                    .or_insert(1);
            }
        }

        field_columns.insert(&*rule.field, possible_columns);
    }

    let mut result = HashMap::new();

    let mut queue = VecDeque::new();
    fields_in_degree.iter().for_each(|(a, b)| {
        if *b == 1 {
            queue.push_back(a.as_str());
        }
    });

    while let Some(current_field) = queue.pop_front() {
        let map_entry = field_columns.remove(current_field).unwrap();
        let column = *map_entry.iter().next().unwrap();
        result.insert(current_field, column);

        for (field, mut columns) in &mut field_columns {
            columns.remove(&column);

            if columns.len() == 1 {
                queue.push_back(*field);
            }
        }
    }

    result
}

pub fn solve_part2() {
    let (rules, ticket, nearby_tickets) = read_input();

    let valid_nearby_tickets: Vec<&Ticket> = nearby_tickets
        .iter()
        .filter(|t| t.values.iter().all(|v| is_valid(*v, &rules)))
        .collect();

    let field_columns = find_field_order(&valid_nearby_tickets, &rules);

    for entry in &field_columns {
        println!("{:?}", entry);
    }

    let result: u64 = field_columns
        .iter()
        .filter(|(f, _)| f.starts_with("departure"))
        .map(|(_, c)| ticket.values[*c])
        .product();

    println!("the result is {}", result);
}
