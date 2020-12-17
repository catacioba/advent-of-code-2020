use std::fmt;

enum Tile {
    Inactive,
    Active,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Inactive => {
                write!(f, ".")
            }
            Tile::Active => {
                write!(f, "#")
            }
        }
    }
}

mod state3 {
    use crate::ch17::Tile;
    use std::collections::VecDeque;
    use std::{fmt, fs};

    pub(crate) struct State {
        grid: VecDeque<VecDeque<VecDeque<Tile>>>,
        x: i16,
        y: i16,
        z: i16,
    }

    impl State {
        pub(crate) fn from_file(filename: &str) -> State {
            let initial_grid: VecDeque<VecDeque<Tile>> = fs::read_to_string(filename)
                .expect("Could not find input file!")
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '#' => Tile::Active,
                            '.' => Tile::Inactive,
                            _ => {
                                panic!("Invalid tile {}", c);
                            }
                        })
                        .collect()
                })
                .collect();

            let mut initial_state_grid = VecDeque::new();
            initial_state_grid.push_back(initial_grid);

            let mut state = State {
                grid: Default::default(),
                x: 0,
                y: 0,
                z: 0,
            };
            state.update_grid(initial_state_grid);

            state
        }

        fn update_grid(&mut self, new_grid: VecDeque<VecDeque<VecDeque<Tile>>>) {
            self.x = new_grid[0].len() as i16;
            self.y = new_grid[0][0].len() as i16;
            self.z = new_grid.len() as i16;
            self.grid = new_grid;
        }

        fn tile(&self, x: i16, y: i16, z: i16) -> &Tile {
            if x >= 0 && x < self.x && y >= 0 && y < self.y && z >= 0 && z < self.z {
                &self.grid[z as usize][x as usize][y as usize]
            } else {
                &Tile::Inactive
            }
        }

        fn count_neighbors(&self, x: i16, y: i16, z: i16) -> u32 {
            let mut accum = 0;
            for dx in -1..2 {
                for dy in -1..2 {
                    for dz in -1..2 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        if let Tile::Active = self.tile(x + dx, y + dy, z + dz) {
                            accum += 1;
                        }
                    }
                }
            }
            accum
        }

        fn new_tile(&self, x: i16, y: i16, z: i16) -> Tile {
            let occupied_neighbors = self.count_neighbors(x, y, z);
            let tile = self.tile(x, y, z);

            match tile {
                Tile::Active => {
                    if occupied_neighbors == 2 || occupied_neighbors == 3 {
                        Tile::Active
                    } else {
                        Tile::Inactive
                    }
                }
                Tile::Inactive => {
                    if occupied_neighbors == 3 {
                        Tile::Active
                    } else {
                        Tile::Inactive
                    }
                }
            }
        }

        pub(crate) fn iterate(&mut self) {
            let mut new_grid: VecDeque<VecDeque<VecDeque<Tile>>> = VecDeque::new();
            for z in -1..(self.z + 1) {
                let mut new_level = VecDeque::new();
                for x in -1..(self.x + 1) {
                    let mut new_row = VecDeque::new();
                    for y in -1..(self.y + 1) {
                        new_row.push_back(self.new_tile(x, y, z))
                    }
                    new_level.push_back(new_row)
                }
                new_grid.push_back(new_level);
            }
            self.update_grid(new_grid);
        }

        pub(crate) fn count_active(&self) -> u32 {
            self.grid
                .iter()
                .map(|l| -> u32 {
                    l.iter()
                        .map(|r| -> u32 {
                            r.iter()
                                .map(|c| if let Tile::Active = c { 1 } else { 0 })
                                .sum()
                        })
                        .sum()
                })
                .sum()
        }
    }

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let middle = self.z / 2;
            for z in 0..self.z {
                let current_z = z - middle;
                writeln!(f, "z={}", current_z);

                let level_grid = &self.grid[z as usize];
                for row in level_grid {
                    for column in row {
                        write!(f, "{}", column);
                    }
                    writeln!(f);
                }
            }
            fmt::Result::Ok(())
        }
    }
}

pub fn solve_part1() {
    let mut grid = state3::State::from_file("src/ch17/input.txt");

    println!("After 0 cycles:\n{}", grid);
    for it in 0..6 {
        grid.iterate();
        println!("After {} cycles:\n{}", it + 1, grid);
        println!("Active cubes: {}", grid.count_active());
    }
}

mod state4 {
    use crate::ch17::Tile;
    use std::collections::VecDeque;
    use std::{fmt, fs};

    pub(crate) struct State {
        grid: VecDeque<VecDeque<VecDeque<VecDeque<Tile>>>>,
        x: i16,
        y: i16,
        z: i16,
        w: i16,
    }

    impl State {
        pub(crate) fn from_file(filename: &str) -> State {
            let initial_grid: VecDeque<VecDeque<Tile>> = fs::read_to_string(filename)
                .expect("Could not find input file!")
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '#' => Tile::Active,
                            '.' => Tile::Inactive,
                            _ => {
                                panic!("Invalid tile {}", c);
                            }
                        })
                        .collect()
                })
                .collect();

            let mut initial_state3_grid = VecDeque::new();
            initial_state3_grid.push_back(initial_grid);
            let mut initial_state4_grid = VecDeque::new();
            initial_state4_grid.push_back(initial_state3_grid);

            let mut state = State {
                grid: Default::default(),
                x: 0,
                y: 0,
                z: 0,
                w: 0,
            };
            state.update_grid(initial_state4_grid);

            state
        }

        fn update_grid(&mut self, new_grid: VecDeque<VecDeque<VecDeque<VecDeque<Tile>>>>) {
            self.x = new_grid[0][0].len() as i16;
            self.y = new_grid[0][0][0].len() as i16;
            self.z = new_grid.len() as i16;
            self.w = new_grid[0].len() as i16;
            self.grid = new_grid;
        }

        fn tile(&self, x: i16, y: i16, z: i16, w: i16) -> &Tile {
            if x >= 0
                && x < self.x
                && y >= 0
                && y < self.y
                && z >= 0
                && z < self.z
                && w >= 0
                && w < self.w
            {
                &self.grid[z as usize][w as usize][x as usize][y as usize]
            } else {
                &Tile::Inactive
            }
        }

        fn count_neighbors(&self, x: i16, y: i16, z: i16, w: i16) -> u32 {
            let mut accum = 0;
            for dx in -1..2 {
                for dy in -1..2 {
                    for dz in -1..2 {
                        for dw in -1..2 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }
                            if let Tile::Active = self.tile(x + dx, y + dy, z + dz, w + dw) {
                                accum += 1;
                            }
                        }
                    }
                }
            }
            accum
        }

        fn new_tile(&self, x: i16, y: i16, z: i16, w: i16) -> Tile {
            let occupied_neighbors = self.count_neighbors(x, y, z, w);
            let tile = self.tile(x, y, z, w);

            match tile {
                Tile::Active => {
                    if occupied_neighbors == 2 || occupied_neighbors == 3 {
                        Tile::Active
                    } else {
                        Tile::Inactive
                    }
                }
                Tile::Inactive => {
                    if occupied_neighbors == 3 {
                        Tile::Active
                    } else {
                        Tile::Inactive
                    }
                }
            }
        }

        pub(crate) fn iterate(&mut self) {
            let mut new_grid4 = VecDeque::new();
            for z in -1..(self.z + 1) {
                let mut new_grid3 = VecDeque::new();
                for w in -1..(self.w + 1) {
                    let mut new_level = VecDeque::new();
                    for x in -1..(self.x + 1) {
                        let mut new_row = VecDeque::new();
                        for y in -1..(self.y + 1) {
                            new_row.push_back(self.new_tile(x, y, z, w))
                        }
                        new_level.push_back(new_row)
                    }
                    new_grid3.push_back(new_level);
                }
                new_grid4.push_back(new_grid3);
            }
            self.update_grid(new_grid4);
        }

        pub(crate) fn count_active(&self) -> u32 {
            self.grid
                .iter()
                .map(|l4| -> u32 {
                    l4.iter()
                        .map(|l3| -> u32 {
                            l3.iter()
                                .map(|r| -> u32 {
                                    r.iter()
                                        .map(|c| if let Tile::Active = c { 1 } else { 0 })
                                        .sum()
                                })
                                .sum()
                        })
                        .sum()
                })
                .sum()
        }
    }
}

pub fn solve_part2() {
    let mut grid = state4::State::from_file("src/ch17/input.txt");

    for it in 0..6 {
        grid.iterate();
    }

    println!("Active cubes: {}", grid.count_active());
}
