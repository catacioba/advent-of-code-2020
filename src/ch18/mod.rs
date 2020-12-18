use std::collections::VecDeque;
use std::{fmt, fs};

fn count_starting_parenthesis(s: &str) -> usize {
    let mut cnt = 0;

    for c in s.chars() {
        if c == '(' {
            cnt += 1
        } else {
            return cnt;
        }
    }

    cnt
}

fn strip_starting_chars(s: &str, c: usize) -> &str {
    &s[c..]
}

fn strip_ending_chars(s: &str, c: usize) -> &str {
    &s[..s.len() - c]
}

fn count_ending_paranthesis(s: &str) -> usize {
    let mut cnt = 0;

    for c in s.chars().rev() {
        if c == ')' {
            cnt += 1
        } else {
            return cnt;
        }
    }

    cnt
}

#[derive(Debug, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug)]
enum Operand {
    Number(i64),
    Expression(Expression),
}

impl Operand {
    fn evaluate(&mut self) -> i64 {
        match self {
            Operand::Number(n) => *n,
            Operand::Expression(e) => e.evaluate(),
        }
    }

    fn evaluate_with_different_precedence(&mut self) -> i64 {
        match self {
            Operand::Number(n) => *n,
            Operand::Expression(e) => e.evaluate_with_different_precedence(),
        }
    }
}

#[derive(Debug)]
struct Expression {
    operands: VecDeque<Operand>,
    operations: VecDeque<Operation>,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            operands: VecDeque::new(),
            operations: VecDeque::new(),
        }
    }

    fn add_operand(&mut self, op: Operand) {
        self.operands.push_back(op)
    }

    fn add_operation(&mut self, op: Operation) {
        self.operations.push_back(op);
    }

    fn parse(expr: &str) -> Expression {
        let mut stack = VecDeque::new();
        stack.push_back(Expression::new());

        expr.split(" ").for_each(|t| {
            let levels_to_open = count_starting_parenthesis(t);
            let levels_to_close = count_ending_paranthesis(t);

            if levels_to_open > 0 {
                for _ in 0..levels_to_open {
                    stack.push_back(Expression::new());
                }

                let token = strip_starting_chars(t, levels_to_open);

                stack.back_mut().unwrap().add_operand(Operand::Number(
                    token.parse().expect("Could not parse operand!"),
                ));
            } else if levels_to_close > 0 {
                let token = strip_ending_chars(t, levels_to_close);

                stack.back_mut().unwrap().add_operand(Operand::Number(
                    token.parse().expect("Could not parse operand!"),
                ));

                for _ in 0..levels_to_close {
                    let last = stack.pop_back().unwrap();
                    stack
                        .back_mut()
                        .unwrap()
                        .add_operand(Operand::Expression(last));
                }
            } else {
                match t {
                    "+" => stack.back_mut().unwrap().add_operation(Operation::Addition),
                    "*" => stack
                        .back_mut()
                        .unwrap()
                        .add_operation(Operation::Multiplication),
                    x => stack.back_mut().unwrap().add_operand(Operand::Number(
                        x.parse().expect("Could not parse operand!"),
                    )),
                }
            }
        });

        stack.pop_front().unwrap()
    }

    fn evaluate_one(&mut self) -> i64 {
        let a = self.operands.pop_front().unwrap().evaluate();
        let b = self.operands.pop_front().unwrap().evaluate();

        match &self.operations.pop_front().unwrap() {
            Operation::Addition => a + b,
            Operation::Multiplication => a * b,
        }
    }

    fn evaluate(&mut self) -> i64 {
        while self.operands.len() > 2 {
            let c = self.evaluate_one();
            self.operands.push_front(Operand::Number(c));
        }
        self.evaluate_one()
    }

    fn evaluate_one_with_different_precedence(&mut self) -> i64 {
        let a = self
            .operands
            .pop_front()
            .unwrap()
            .evaluate_with_different_precedence();
        let b = self
            .operands
            .pop_front()
            .unwrap()
            .evaluate_with_different_precedence();

        match &self.operations.pop_front().unwrap() {
            Operation::Addition => a + b,
            Operation::Multiplication => a * b,
        }
    }

    fn evaluate_with_different_precedence(&mut self) -> i64 {
        while self.operands.len() > 2 {
            if self.operations.len() > 1
                && self.operations[0] != Operation::Addition
                && self.operations[1] == Operation::Addition
            {
                let first_operand = self.operands.pop_front().unwrap();
                let first_operation = self.operations.pop_front().unwrap();

                let c = self.evaluate_one_with_different_precedence();
                self.operands.push_front(Operand::Number(c));

                self.operands.push_front(first_operand);
                self.operations.push_front(first_operation);
            } else {
                let c = self.evaluate_one_with_different_precedence();
                self.operands.push_front(Operand::Number(c));
            }
        }
        self.evaluate_one_with_different_precedence()
    }
}

fn read_expressions() -> Vec<Expression> {
    fs::read_to_string("src/ch18/input.txt")
        .unwrap()
        .lines()
        .map(|l| Expression::parse(l))
        .collect()
}

pub fn solve_part1() {
    let mut expressions = read_expressions();

    let result: i64 = expressions.iter_mut().map(|e| e.evaluate()).sum();

    println!("result: {}", result);
}

pub fn solve_part2() {
    let mut expressions = read_expressions();

    let result: i64 = expressions
        .iter_mut()
        .map(|e| e.evaluate_with_different_precedence())
        .sum();

    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_starting_parenthesis_works() {
        assert_eq!(count_starting_parenthesis(""), 0);
        assert_eq!(count_starting_parenthesis("()("), 1);
        assert_eq!(count_starting_parenthesis("a("), 0);
        assert_eq!(count_starting_parenthesis("(("), 2);
        assert_eq!(count_starting_parenthesis("(((a"), 3);
    }

    #[test]
    fn strip_starting_chars_works() {
        assert_eq!(strip_starting_chars("", 0), "");
        assert_eq!(strip_starting_chars("()(", 1), ")(");
        assert_eq!(strip_starting_chars("a(", 0), "a(");
        assert_eq!(strip_starting_chars("((", 2), "");
        assert_eq!(strip_starting_chars("(((a", 3), "a");
    }

    #[test]
    fn count_ending_paranthesis_works() {
        assert_eq!(count_ending_paranthesis(""), 0);
        assert_eq!(count_ending_paranthesis("("), 0);
        assert_eq!(count_ending_paranthesis("()"), 1);
        assert_eq!(count_ending_paranthesis(")"), 1);
        assert_eq!(count_ending_paranthesis("))"), 2);
        assert_eq!(count_ending_paranthesis("a)))"), 3);
    }

    #[test]
    fn strip_ending_chars_works() {
        assert_eq!(strip_ending_chars("", 0), "");
        assert_eq!(strip_ending_chars("(", 0), "(");
        assert_eq!(strip_ending_chars("()", 1), "(");
        assert_eq!(strip_ending_chars(")", 1), "");
        assert_eq!(strip_ending_chars("))", 2), "");
        assert_eq!(strip_ending_chars("a)))", 3), "a");
    }

    #[test]
    fn evaluate_expression_no_paranthesis_works() {
        assert_eq!(Expression::parse("1 + 2 * 3 + 4 * 5 + 6").evaluate(), 71);
    }

    #[test]
    fn evaluate_expression_with_parenthesis_depth_one_works() {
        assert_eq!(Expression::parse("2 * 3 + (4 * 5)").evaluate(), 26);
        assert_eq!(
            Expression::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").evaluate(),
            437
        );
    }

    #[test]
    fn evaluate_expression_with_parenthesis_depth_three_works() {
        assert_eq!(
            Expression::parse("1 + (2 * 3) + (4 * (5 + 6))").evaluate(),
            51
        );
        assert_eq!(
            Expression::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").evaluate(),
            12240
        );
        assert_eq!(
            Expression::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").evaluate(),
            13632
        );
    }

    #[test]
    fn evaluate_with_different_precedence_expression_no_paranthesis_works() {
        assert_eq!(
            Expression::parse("1 + 2 * 3 + 4 * 5 + 6").evaluate_with_different_precedence(),
            231
        );
    }

    #[test]
    fn evaluate_with_different_precedence_expression_with_parenthesis_depth_one_works() {
        assert_eq!(
            Expression::parse("2 * 3 + (4 * 5)").evaluate_with_different_precedence(),
            46
        );
        assert_eq!(
            Expression::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").evaluate_with_different_precedence(),
            1445
        );
    }

    #[test]
    fn evaluate_with_different_precedence_expression_with_parenthesis_depth_three_works() {
        assert_eq!(
            Expression::parse("1 + (2 * 3) + (4 * (5 + 6))").evaluate_with_different_precedence(),
            51
        );
        assert_eq!(
            Expression::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .evaluate_with_different_precedence(),
            669060
        );
        assert_eq!(
            Expression::parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .evaluate_with_different_precedence(),
            23340
        );
    }
}
