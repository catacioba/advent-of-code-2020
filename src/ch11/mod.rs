use std::fs;

mod room {
    use std::fmt;
    use std::fmt::Formatter;

    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    #[derive(PartialEq, Copy, Clone)]
    enum Tile {
        Floor,
        Empty,
        Occupied,
    }

    impl Tile {
        fn from_char(c: char) -> Tile {
            match c {
                '.' => Tile::Floor,
                'L' => Tile::Empty,
                '#' => Tile::Occupied,
                _ => {
                    panic!("invalid tile: {}", c);
                }
            }
        }
    }

    pub(crate) struct Room {
        map: Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        strategy: NearbyTileStrategy,
    }

    pub(crate) enum NearbyTileStrategy {
        Adjacent,
        Visible,
    }

    impl Room {
        pub(crate) fn from_lines(str: String, strategy: NearbyTileStrategy) -> Room {
            let map: Vec<Vec<Tile>> = str
                .lines()
                .map(|l| l.chars().map(|c| Tile::from_char(c)).collect())
                .collect();
            Room {
                width: map[0].len(),
                height: map.len(),
                map,
                strategy,
            }
        }

        pub(crate) fn simulate(&mut self) {
            let mut iteration = 1;

            while !self.iterate() {
                iteration += 1;
            }

            println!("Stabilized after {} iterations", iteration);
        }

        pub(crate) fn count_occupied_seats(&self) -> usize {
            self.map
                .iter()
                .map(|r| r.iter().filter(|&t| *t == Tile::Occupied).count())
                .sum()
        }

        fn iterate(&mut self) -> bool {
            let mut new_map = Vec::new();

            for (x, row) in self.map.iter().enumerate() {
                let mut new_row = Vec::new();

                for (y, _) in row.iter().enumerate() {
                    match self.strategy {
                        NearbyTileStrategy::Adjacent => {
                            new_row.push(self.adjacent_tiles_strategy(x, y));
                        }
                        NearbyTileStrategy::Visible => {
                            new_row.push(self.nearest_tiles_strategy(x, y));
                        }
                    }
                }

                new_map.push(new_row);
            }

            let has_map_stabilized = self.map == new_map;

            self.map = new_map;

            has_map_stabilized
        }

        fn adjacent_tiles_strategy(&self, x: usize, y: usize) -> Tile {
            let tile = self.map[x][y];
            let nearby_occupied_seats = self.get_adjacent_tiles(x, y);

            if tile == Tile::Empty && nearby_occupied_seats == 0 {
                Tile::Occupied
            } else if tile == Tile::Occupied && nearby_occupied_seats >= 4 {
                Tile::Empty
            } else {
                tile
            }
        }

        fn get_adjacent_tiles(&self, x: usize, y: usize) -> usize {
            DIRECTIONS
                .iter()
                .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                .filter(|&(x, y)| {
                    self.in_bounds(x, y) && self.map[x as usize][y as usize] == Tile::Occupied
                })
                .count()
        }

        fn nearest_tiles_strategy(&self, x: usize, y: usize) -> Tile {
            let tile = self.map[x][y];
            let nearest_occupied_seats = self.get_nearest_tiles(x, y);

            if tile == Tile::Empty && nearest_occupied_seats == 0 {
                Tile::Occupied
            } else if tile == Tile::Occupied && nearest_occupied_seats >= 5 {
                Tile::Empty
            } else {
                tile
            }
        }

        fn get_nearest_tiles(&self, x: usize, y: usize) -> usize {
            DIRECTIONS
                .iter()
                .filter(|(dx, dy)| {
                    let mut curr_x = x as i32 + dx;
                    let mut curr_y = y as i32 + dy;

                    while self.in_bounds(curr_x, curr_y) {
                        let tile = self.map[curr_x as usize][curr_y as usize];
                        if tile != Tile::Floor {
                            return tile == Tile::Occupied;
                        }

                        curr_x += dx;
                        curr_y += dy;
                    }

                    false
                })
                .count()
        }

        fn in_bounds(&self, x: i32, y: i32) -> bool {
            x >= 0 && x < self.height as i32 && y >= 0 && y < self.width as i32
        }
    }

    impl fmt::Display for Room {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for row in &self.map {
                for tile in row {
                    match tile {
                        Tile::Floor => write!(f, "."),
                        Tile::Empty => write!(f, "L"),
                        Tile::Occupied => write!(f, "#"),
                    };
                }
                writeln!(f);
            }
            Ok(())
        }
    }
}

pub fn solve_part1() {
    let mut room = room::Room::from_lines(
        fs::read_to_string("src/ch11/input.txt").unwrap(),
        room::NearbyTileStrategy::Adjacent,
    );

    room.simulate();

    println!("occupied seats: {}", room.count_occupied_seats());
}

pub fn solve_part2() {
    let mut room = room::Room::from_lines(
        fs::read_to_string("src/ch11/input.txt").unwrap(),
        room::NearbyTileStrategy::Visible,
    );

    room.simulate();

    println!("occupied seats: {}", room.count_occupied_seats());
}
