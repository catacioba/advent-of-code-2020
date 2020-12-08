use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    let (o, p) = line.split_at(4);

    let op = o.trim_end();
    let param: i32 = p.parse().unwrap();

    match op {
        "nop" => Instruction::Nop(param),
        "acc" => Instruction::Acc(param),
        "jmp" => Instruction::Jmp(param),
        _ => panic!("Unknown operatin {}", op),
    }
}

fn parse_instructions() -> Vec<Instruction> {
    fs::read_to_string("src/ch08/input.txt")
        .unwrap()
        .lines()
        .map(|l| parse_instruction(l))
        .collect()
}

#[derive(Debug)]
enum Result {
    Failed,
    InfiniteLoop(i32),
    Normal(i32),
}

fn compute_accumulator(instructions: &Vec<Instruction>) -> Result {
    let mut accum = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut it: i32 = 0;

    while it < instructions.len() as i32 {
        if it < 0 {
            return Result::Failed;
        }

        let instr = &instructions[it as usize];

        // println!("on step {}: {:?} | visited: {:?}", it, instr, &visited);

        if visited.contains(&it) {
            return Result::InfiniteLoop(accum);
        }
        visited.insert(it);

        match instr {
            Instruction::Nop(_) => it += 1,
            Instruction::Acc(num) => {
                accum += num;
                it += 1
            }
            Instruction::Jmp(num) => it += num,
        }
    }

    Result::Normal(accum)
}

pub fn solve_part1() {
    let instructions = parse_instructions();

    let accum = compute_accumulator(&instructions);

    println!("accumulator: {:?}", accum);
}

fn get_alternate_instruction(instr: &Instruction) -> Instruction {
    match instr {
        Instruction::Nop(num) => Instruction::Jmp(*num),
        Instruction::Jmp(num) => Instruction::Nop(*num),
        Instruction::Acc(num) => Instruction::Acc(*num),
    }
}

fn search_correct_result(instructions: &mut Vec<Instruction>) -> i32 {
    let len = instructions.len();

    for idx in 0..len {
        let current_instruction = &instructions[idx];

        let alternate_instruction = get_alternate_instruction(current_instruction);

        instructions[idx] = alternate_instruction;

        if let Result::Normal(accum) = compute_accumulator(&instructions) {
            return accum;
        }

        instructions[idx] = get_alternate_instruction(&alternate_instruction);
    }
    panic!("No solution found!");
}

pub fn solve_part2() {
    let mut instructions = parse_instructions();

    let accum = search_correct_result(&mut instructions);

    println!("accumulator: {}", accum);
}
