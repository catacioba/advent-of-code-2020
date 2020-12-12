use std::fs;

mod navigation {
    use std::collections::HashMap;

    #[derive(Copy, Clone)]
    pub enum Direction {
        South,
        North,
        West,
        East,
        Forward,
    }

    pub enum Rotation {
        Right,
        Left,
    }

    pub enum Step {
        Direction(Direction),
        Rotation(Rotation),
    }

    impl Step {
        fn from_char(c: char) -> Step {
            match c {
                'N' => Step::Direction(Direction::North),
                'S' => Step::Direction(Direction::South),
                'E' => Step::Direction(Direction::East),
                'W' => Step::Direction(Direction::West),
                'F' => Step::Direction(Direction::Forward),
                'L' => Step::Rotation(Rotation::Left),
                'R' => Step::Rotation(Rotation::Right),
                _ => panic!("Unrecognized navigation direction {}", c),
            }
        }
    }

    pub(crate) struct Instruction {
        pub(crate) step: Step,
        pub(crate) amount: i32,
    }

    impl Instruction {
        pub(crate) fn from_str(line: &str) -> Instruction {
            Instruction {
                step: Step::from_char(line.chars().next().unwrap()),
                amount: line[1..].parse::<i32>().unwrap(),
            }
        }
    }

    pub(crate) struct State {
        east: i32,
        north: i32,
        direction_states: Vec<Direction>,
        current_direction: usize,
    }

    impl State {
        pub(crate) fn new() -> State {
            State {
                east: 0,
                north: 0,
                direction_states: vec![
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ],
                current_direction: 1,
            }
        }

        fn current_direction(&self) -> Direction {
            self.direction_states[self.current_direction]
        }

        fn rotate_left(&mut self, angles: i32) {
            let position_change = angles / 90;
            self.current_direction =
                ((4 + self.current_direction as i32 - position_change) % 4) as usize;
        }

        fn rotate_right(&mut self, angles: i32) {
            let position_change = angles / 90;
            self.current_direction =
                ((self.current_direction as i32 + position_change) % 4) as usize;
        }

        fn move_boat(&mut self, direction: Direction, distance: i32) {
            match direction {
                Direction::South => {
                    self.north -= distance;
                }
                Direction::North => {
                    self.north += distance;
                }
                Direction::West => {
                    self.east -= distance;
                }
                Direction::East => {
                    self.east += distance;
                }
                Direction::Forward => {
                    self.move_boat(self.current_direction(), distance);
                }
            }
        }

        pub(crate) fn step(&mut self, instruction: Instruction) {
            match instruction.step {
                Step::Direction(direction) => {
                    self.move_boat(direction, instruction.amount);
                }
                Step::Rotation(rotation) => match rotation {
                    Rotation::Right => {
                        self.rotate_right(instruction.amount);
                    }
                    Rotation::Left => {
                        self.rotate_left(instruction.amount);
                    }
                },
            }
        }

        pub(crate) fn distance(&self) -> i32 {
            self.east.abs() + self.north.abs()
        }
    }
}

pub fn solve_part1() {
    let instructions: Vec<navigation::Instruction> = fs::read_to_string("src/ch12/input.txt")
        .unwrap()
        .lines()
        .map(|l| navigation::Instruction::from_str(l))
        .collect();

    let mut state = navigation::State::new();

    for instruction in instructions {
        state.step(instruction);
    }

    println!("Distance: {}", state.distance());
}

pub fn solve_part2() {
    let instructions: Vec<navigation::Instruction> = fs::read_to_string("src/ch12/input.txt")
        .unwrap()
        .lines()
        .map(|l| navigation::Instruction::from_str(l))
        .collect();

    let mut ship_east = 0;
    let mut ship_north = 0;

    let mut waypoint_east = 10;
    let mut waypoint_north = 1;

    for instruction in instructions {
        match instruction.step {
            navigation::Step::Direction(direction) => match direction {
                navigation::Direction::South => {
                    waypoint_north -= instruction.amount;
                }
                navigation::Direction::North => {
                    waypoint_north += instruction.amount;
                }
                navigation::Direction::West => {
                    waypoint_east -= instruction.amount;
                }
                navigation::Direction::East => {
                    waypoint_east += instruction.amount;
                }
                navigation::Direction::Forward => {
                    ship_east += instruction.amount * waypoint_east;
                    ship_north += instruction.amount * waypoint_north;
                }
            },
            navigation::Step::Rotation(rotation) => {
                let amount = instruction.amount / 90;
                let tmp_east = waypoint_east;
                let tmp_north = waypoint_north;

                match rotation {
                    navigation::Rotation::Right => match amount {
                        1 => {
                            waypoint_east = tmp_north;
                            waypoint_north = -tmp_east;
                        }
                        2 => {
                            waypoint_east = -tmp_east;
                            waypoint_north = -tmp_north;
                        }
                        3 => {
                            waypoint_east = -tmp_north;
                            waypoint_north = tmp_east;
                        }
                        _ => {
                            panic!("unrecognized rotation {}", instruction.amount);
                        }
                    },
                    navigation::Rotation::Left => match amount {
                        1 => {
                            waypoint_east = -tmp_north;
                            waypoint_north = tmp_east;
                        }
                        2 => {
                            waypoint_east = -tmp_east;
                            waypoint_north = -tmp_north;
                        }
                        3 => {
                            waypoint_east = tmp_north;
                            waypoint_north = -tmp_east;
                        }
                        _ => {
                            panic!("unrecognized rotation {}", instruction.amount);
                        }
                    },
                }
            }
        }
    }

    println!("distance: {}", ship_north.abs() + ship_east.abs());
}
