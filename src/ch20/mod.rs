use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Pixel {
    Black, // '#'
    White, // '.'
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum BorderPosition {
    Top,
    Right,
    Bottom,
    Left,
}

impl BorderPosition {
    fn is_reverse(&self) -> bool {
        match self {
            BorderPosition::Top => false,
            BorderPosition::Right => false,
            BorderPosition::Bottom => true,
            BorderPosition::Left => true,
        }
    }
}

const ALL_BORDER_POSITIONS: [BorderPosition; 4] = [
    BorderPosition::Top,
    BorderPosition::Right,
    BorderPosition::Bottom,
    BorderPosition::Left,
];

type Border = Vec<Pixel>;
type TileId = u32;

#[derive(Copy, Clone, Debug)]
enum Rotation {
    None,
    One,
    Two,
    Three,
}

const ALL_ROTATIONS: [Rotation; 4] = [
    Rotation::None,
    Rotation::One,
    Rotation::Two,
    Rotation::Three,
];

struct Tile {
    id: TileId,
    pixels: Vec<Vec<Pixel>>,
    size: usize,
}

fn parse_tile_id(line: &str) -> u32 {
    line[5..line.len() - 1]
        .parse()
        .expect("Expected tile id to be a number")
}

impl Tile {
    fn from_block(block: &str) -> Tile {
        let mut lines = block.lines();

        let id = parse_tile_id(lines.next().expect("Expected to have tile id line"));

        let pixels: Vec<Vec<Pixel>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Pixel::White,
                        '#' => Pixel::Black,
                        _ => panic!("Invalid tile character {}", c),
                    })
                    .collect()
            })
            .collect();

        let size = pixels.len();

        Tile { id, pixels, size }
    }

    // return the border with the given position AFTER applying the given rotation.
    fn border(&self, position: BorderPosition, rotation: Rotation) -> Border {
        let (border_position, reverse) = self.border_rotation(position, rotation);
        let mut border = match border_position {
            BorderPosition::Top => self.pixels.iter().next().unwrap().to_owned(),
            BorderPosition::Right => self
                .pixels
                .iter()
                .map(|r| *r.last().expect("Empty tile row"))
                .collect(),
            BorderPosition::Bottom => self
                .pixels
                .iter()
                .last()
                .unwrap()
                .iter()
                .rev()
                .cloned()
                .collect(),
            BorderPosition::Left => self.pixels.iter().rev().map(|r| r[0]).collect(),
        };
        if reverse {
            border.reverse()
        }
        border
    }

    fn border_rotation(&self, position: BorderPosition, r: Rotation) -> (BorderPosition, bool) {
        let num_rotations = match r {
            Rotation::None => 0,
            Rotation::One => 1,
            Rotation::Two => 2,
            Rotation::Three => 3,
        };
        let pos = ALL_BORDER_POSITIONS
            .iter()
            .position(|&x| x == position)
            .unwrap();
        let new_position = ALL_BORDER_POSITIONS[(pos + num_rotations) % 4];
        let reverse = position.is_reverse() ^ new_position.is_reverse();

        (new_position, reverse)
    }
}

fn group_borders(
    tiles: &Vec<Tile>,
) -> HashMap<Border, HashMap<TileId, Vec<(BorderPosition, Rotation)>>> {
    let mut borders_by_tiles = HashMap::new();

    for tile in tiles {
        for border_position in &ALL_BORDER_POSITIONS {
            for rotation in &ALL_ROTATIONS {
                let border = tile.border(*border_position, *rotation);
                let matching_tiles = borders_by_tiles.entry(border).or_insert(HashMap::new());
                let orientations = matching_tiles.entry(tile.id).or_insert(Vec::new());
                orientations.push((*border_position, *rotation));
            }
        }
    }

    borders_by_tiles
}

fn compute_matching_tiles(
    tiles: &Vec<Tile>,
    borders_by_tiles: &HashMap<Border, HashMap<TileId, Vec<(BorderPosition, Rotation)>>>,
) -> HashMap<TileId, HashMap<BorderPosition, Vec<(TileId, BorderPosition, Rotation)>>> {
    let mut matching_tiles = HashMap::new();

    for tile in tiles {
        let mut m = HashMap::new();

        for border_position in &ALL_BORDER_POSITIONS {
            let border = tile.border(*border_position, Rotation::None);
            m.insert(
                *border_position,
                borders_by_tiles
                    .get(&border)
                    .unwrap()
                    .iter()
                    .filter(|(&tile_id, _)| tile_id != tile.id)
                    .map(|(&t, x)| x.iter().map(move |&(p, r)| (t, p, r)))
                    .flatten()
                    .collect(),
            );
        }

        matching_tiles.insert(tile.id, m);
    }

    matching_tiles
}

struct Image {
    size: usize,
    tiles: Vec<Vec<(TileId, Rotation)>>,
}

fn solve_dumb(tiles: &Vec<Tile>) {
    let size = (tiles.len() as f32).sqrt() as usize;
    println!("trying to reconstruct an image of size {}x{}", size, size);

    let borders_by_tiles = group_borders(tiles);

    for (k, v) in &borders_by_tiles {
        // if v.len() > 1 {
        for (x, y) in v {
            println!("{} => {:?}", x, y);
        }
        // }
    }

    let matching_tiles = compute_matching_tiles(tiles, &borders_by_tiles);

    for (tile_id, borders) in &matching_tiles {
        println!("For tile {}", tile_id);
        for (border_pos, tiles) in borders {
            println!("\t{:?} => {:?}", border_pos, tiles);
        }
    }

    // Tiles should have the following properties:
    //     - if it is a corner, then at least 2 borders match other tiles
    //     - if it is on the edge, then at least 3 borders should match other tiles
    //     - if is is a center tile, then all four borders should match other tiles
    //
    // This can be used to reduce the number of possibilities in a backtracking solution.
    for tile in tiles {}
}

// Checks if a tile can be put in the given position in the final image.
// In other words, it looks if there are other tiles that can match its borders).
fn is_valid_for_position(
    tile: &Tile,
    rotation: Rotation,
    image_x: usize,
    image_y: usize,
    image_size: usize,
) -> bool {
    let last = image_size - 1;

    if image_x == 0 {
        // Top border. Should match bottom border.
    } else if image_x == last {
        // Bottom border. Should match top border.
    } else {
        // Middle. Should match top and bottom borders.
    }

    if image_y == 0 {
        // Left border. Should match right border
    } else if image_y == last {
        // Right border. Should match left border
    } else {
        // In the middle. Should match both left and right borders
    }

    true
}

fn read_tiles() -> Vec<Tile> {
    fs::read_to_string("src/ch20/input.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|b| Tile::from_block(b))
        .collect()
}

pub fn solve_part1() {
    let tiles = read_tiles();
    println!("Read {} tiles", tiles.len());

    solve_dumb(&tiles);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_block() {
        /*
               #.#
               ..#
               ##.
        */
        let tile = Tile::from_block("Tile 5:\r\n#.#\r\n..#\r\n##.");

        assert_eq!(tile.size, 3);
        assert_eq!(tile.id, 5);
        assert_eq!(
            tile.pixels,
            vec![
                vec![Pixel::Black, Pixel::White, Pixel::Black],
                vec![Pixel::White, Pixel::White, Pixel::Black],
                vec![Pixel::Black, Pixel::Black, Pixel::White]
            ]
        );
    }

    // #[test]
    // fn test_tile_borders() {
    //     /*
    //            #.#
    //            ..#
    //            ##.
    //     */
    //     let tile = Tile::from_block("Tile 5:\r\n#.#\r\n..#\r\n##.");
    //
    //     assert_eq!(
    //         tile.border(BorderPosition::Top),
    //         vec![Pixel::Black, Pixel::White, Pixel::Black]
    //     );
    //     assert_eq!(
    //         tile.border(BorderPosition::Right),
    //         vec![Pixel::Black, Pixel::Black, Pixel::White]
    //     );
    //     assert_eq!(
    //         tile.border(BorderPosition::Bottom),
    //         vec![Pixel::White, Pixel::Black, Pixel::Black]
    //     );
    //     assert_eq!(
    //         tile.border(BorderPosition::Left),
    //         vec![Pixel::Black, Pixel::White, Pixel::Black]
    //     );
    // }
}
