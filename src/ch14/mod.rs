use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Value(u64, u64),
}

fn extract_memory_location(s: &str) -> u64 {
    let slice = &s[4..s.len() - 1];
    slice
        .parse()
        .expect(format!("Failed to parse {}", slice).as_ref())
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        if line.starts_with("mask") {
            let mask = line.split_at(7).1;
            Instruction::Mask(String::from(mask))
        } else if line.starts_with("mem") {
            let mut parts = line.split(" = ");

            let memory_location =
                extract_memory_location(parts.next().expect("Expected memory location part"));
            let value = parts
                .next()
                .expect("Expected value part")
                .parse()
                .expect("Failed to parse value part");

            Instruction::Value(memory_location, value)
        } else {
            panic!("Invalid instruction: {}", line);
        }
    }
}

fn to_binary_string(value: u64) -> String {
    format!("{:0>36b}", value)
}

fn from_binary_string(value: &str) -> u64 {
    u64::from_str_radix(value, 2)
        .expect(format!("Could not convert binary string: {}", value).as_str())
}

fn apply_mask(mask: &str, value: u64) -> u64 {
    let binary_string = to_binary_string(value);

    let masked: String = binary_string
        .chars()
        .zip(mask.chars())
        .map(
            |(char, mask_char)| {
                if mask_char != 'X' {
                    mask_char
                } else {
                    char
                }
            },
        )
        .collect();

    from_binary_string(masked.as_str())
}

fn read_input() -> Vec<Instruction> {
    fs::read_to_string("src/ch14/input.txt")
        .expect("File not found")
        .lines()
        .map(|l| Instruction::from_line(l))
        .collect()
}

pub fn solve_part1() {
    let instructions = read_input();
    let mut memory = HashMap::new();
    let mut current_mask = "";

    instructions.iter().for_each(|instr| match instr {
        Instruction::Mask(mask) => {
            current_mask = mask;
        }
        Instruction::Value(memory_location, value) => {
            memory.insert(memory_location, apply_mask(current_mask, *value));
        }
    });

    let sum: u64 = memory.values().sum();

    println!("sum: {}", sum);
}

fn apply_mask_floating(mask: &str, value: u64) -> String {
    let binary_string = to_binary_string(value);

    let masked_floating: String = binary_string
        .chars()
        .zip(mask.chars())
        .map(|(char, mask_char)| {
            if mask_char == '0' {
                char
            } else if mask_char == '1' {
                '1'
            } else {
                'X'
            }
        })
        .collect();

    masked_floating
}

fn generate_locations(value: &str) -> Vec<String> {
    let mut locations = vec![String::from("")];

    value.chars().for_each(|c| {
        if c != 'X' {
            locations.iter_mut().for_each(|l| l.push(c));
        } else {
            let mut new_locations = Vec::new();

            locations.iter_mut().for_each(|s| {
                let mut new_location = String::from(s.as_str());
                new_location.push('1');
                new_locations.push(new_location);

                s.push('0');
            });

            locations.append(&mut new_locations);
        }
    });

    locations
}

pub fn solve_part2() {
    let instructions = read_input();
    let mut memory = HashMap::new();
    let mut current_mask = "";

    instructions.iter().for_each(|instr| match instr {
        Instruction::Mask(mask) => {
            current_mask = mask;
        }
        Instruction::Value(memory_location, value) => {
            let floating_memory_location = apply_mask_floating(current_mask, *memory_location);

            let memory_locations = generate_locations(&floating_memory_location);

            memory_locations.iter().for_each(|l| {
                memory.insert(String::from(l), *value);
            });
        }
    });

    let sum: u64 = memory.values().sum();

    println!("sum: {}", sum);
}
