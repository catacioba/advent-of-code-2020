use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::West,
    Direction::SouthWest,
    Direction::SouthEast,
];

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Tile {
    x: i32,
    y: i32,
}

impl Tile {
    fn new(x: i32, y: i32) -> Tile {
        Tile { x: x, y: y }
    }
    fn neighbor(&self, direction: Direction) -> Tile {
        let mut neighbor_x = self.x;
        let mut neighbor_y = self.y;

        match direction {
            Direction::East => {
                neighbor_x += 2;
            }
            Direction::NorthEast => {
                neighbor_x += 1;
                neighbor_y += 1;
            }
            Direction::NorthWest => {
                neighbor_x -= 1;
                neighbor_y += 1;
            }
            Direction::West => {
                neighbor_x -= 2;
            }
            Direction::SouthWest => {
                neighbor_x -= 1;
                neighbor_y -= 1;
            }
            Direction::SouthEast => {
                neighbor_x += 1;
                neighbor_y -= 1;
            }
        }

        Tile {
            x: neighbor_x,
            y: neighbor_y,
        }
    }

    fn evaluate_tile(directions: &Vec<Direction>) -> Tile {
        let mut current = Tile { x: 0, y: 0 };

        for direction in directions {
            current = current.neighbor(*direction);
        }

        current
    }
}

// e se ne w nw sw
fn parse_directions(directions: &str) -> Vec<Direction> {
    let mut result = Vec::new();

    let mut it = directions.chars();
    while let Some(c) = it.next() {
        result.push(match c {
            'e' => Direction::East,
            'w' => Direction::West,
            's' => {
                // read next char
                match it.next() {
                    Some('w') => Direction::SouthWest,
                    Some('e') => Direction::SouthEast,
                    _ => {
                        panic!("Invalid next character");
                    }
                }
            }
            'n' => {
                // read next char
                match it.next() {
                    Some('w') => Direction::NorthWest,
                    Some('e') => Direction::NorthEast,
                    _ => {
                        panic!("Invalid next character");
                    }
                }
            }
            _ => {
                panic!("Invalid character while reading input")
            }
        });
    }

    result
}

fn read_tiles() -> Vec<Tile> {
    fs::read_to_string("src/ch24/input.txt")
        .unwrap()
        .lines()
        .map(|d| parse_directions(d))
        .map(|dl| Tile::evaluate_tile(&dl))
        .collect()
}

fn initial_tiles(tiles: &Vec<Tile>) -> HashMap<Tile, bool> {
    let mut tile_colors: HashMap<Tile, bool> = HashMap::new();
    for tile in tiles {
        tile_colors
            .entry(*tile)
            .and_modify(|black| *black = !*black)
            .or_insert(true);
    }
    tile_colors
}

pub fn solve_part1() {
    let tiles: Vec<Tile> = read_tiles();

    let tile_colors = initial_tiles(&tiles);

    let black_tiles = count_black_tiles(&tile_colors);

    println!("black tiles: {}", black_tiles);
}

pub fn solve_part2() {
    let tiles: Vec<Tile> = read_tiles();

    let mut tile_colors = initial_tiles(&tiles);

    let mut min_x = tiles.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let mut max_x = tiles.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let mut min_y = tiles.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let mut max_y = tiles.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    println!("x => [{}, {}]", min_x, max_x);
    println!("y => [{}, {}]", min_y, max_y);

    println!("day 0 => {}", count_black_tiles(&tile_colors));

    for it in 0..100 {
        let mut new_tile_colors: HashMap<Tile, bool> = HashMap::new();

        let mut new_max_x = max_x;
        let mut new_max_y = max_y;
        let mut new_min_x = min_x;
        let mut new_min_y = min_y;

        for y in min_y - 1..max_y + 2 {
            for x in min_x - 2..max_x + 3 {
                if y % 2 == 0 && x % 2 == 1 {
                    continue;
                } else if y % 2 == 1 && x % 2 == 0 {
                    continue;
                }

                let t = Tile::new(x, y);

                let neighbors = count_black_neighbors_tiles(&t, &tile_colors);
                let is_black = *tile_colors.get(&t).unwrap_or(&false);
                let new_color = match is_black {
                    true => {
                        if neighbors == 0 || neighbors > 2 {
                            false
                        } else {
                            true
                        }
                    }
                    false => {
                        if neighbors == 2 {
                            true
                        } else {
                            false
                        }
                    }
                };

                new_max_x = new_max_x.max(t.x);
                new_max_y = new_max_y.max(t.y);
                new_min_x = new_min_x.min(t.x);
                new_min_y = new_min_y.min(t.y);

                new_tile_colors.insert(t, new_color);
            }
        }
        max_x = new_max_x;
        max_y = new_max_y;
        min_x = new_min_x;
        min_y = new_min_y;

        tile_colors = new_tile_colors;

        if it + 1 <= 10 || (it + 1) % 10 == 0 {
            println!("day {} => {}", it + 1, count_black_tiles(&tile_colors));
        }
    }
}

fn count_black_neighbors_tiles(t: &Tile, tiles: &HashMap<Tile, bool>) -> usize {
    ALL_DIRECTIONS
        .iter()
        .map(|d| t.neighbor(*d))
        .filter(|t| *tiles.get(t).unwrap_or(&false))
        .count()
}

fn count_black_tiles(tiles: &HashMap<Tile, bool>) -> usize {
    tiles.values().filter(|v| **v).count()
}

#[cfg(test)]
mod tests {
    use super::Direction::*;
    use super::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(
            parse_directions("nwwswee"),
            vec![
                Direction::NorthWest,
                Direction::West,
                Direction::SouthWest,
                Direction::East,
                Direction::East
            ]
        );
        assert_eq!(
            parse_directions("esew"),
            vec![Direction::East, Direction::SouthEast, Direction::West],
        )
    }

    #[test]
    fn test_tile_neighbor() {
        assert_eq!(
            Tile::evaluate_tile(&vec![East, NorthWest, SouthWest]),
            Tile { x: 0, y: 0 }
        );
        assert_eq!(
            Tile::evaluate_tile(&vec![NorthWest, West, SouthWest, East, East]),
            Tile { x: 0, y: 0 }
        );
        assert_eq!(
            Tile::evaluate_tile(&vec![East, SouthEast, West]),
            Tile { x: 1, y: -1 }
        );
    }
}
