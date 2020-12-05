use crate::utils::utils::read_lines;

#[derive(Debug)]
enum MapTile {
    Open,
    Tree,
}

#[derive(Debug)]
struct Map {
    pattern: Vec<Vec<MapTile>>,
    height: usize,
    width: usize,
}

struct Slope {
    x: usize,
    y: usize,
}

impl Map {
    fn get_char(&self, x: usize, y: usize) -> &MapTile {
        if x > self.height {
            panic!("Invalid x value");
        }
        &self.pattern[x][y % self.width]
    }

    fn count_tree_for_slope(&self, slope: &Slope) -> i64 {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut cnt = 0;

        while x < self.height {
            if let MapTile::Tree = self.get_char(x, y) {
                cnt += 1;
            }
            x += slope.x;
            y += slope.y;
        }

        cnt
    }
}

fn parse_pattern() -> Vec<Vec<MapTile>> {
    if let Ok(lines) = read_lines("src/ch03/input.txt") {
        lines
            .map(|l| {
                let line = l.unwrap();
                line.chars()
                    .map(|c| match c {
                        '.' => MapTile::Open,
                        '#' => MapTile::Tree,
                        _ => panic!("tile {} not recognized!", c),
                    })
                    .collect()
            })
            .collect()
    } else {
        panic!("File not found");
    }
}

fn parse_map() -> Map {
    let pat = parse_pattern();
    Map {
        height: pat.len(),
        width: pat[0].len(),
        pattern: pat,
    }
}

fn debug_map(m: &Map) {
    println!("height: {} width: {}", m.height, m.width);
    for row in &m.pattern {
        for col in row {
            match col {
                MapTile::Open => print!("."),
                MapTile::Tree => print!("#"),
            }
        }
        println!();
    }
}

pub fn solve_part1() {
    let m = parse_map();
    let s = Slope { x: 1, y: 3 };

    println!("Tree count {}", m.count_tree_for_slope(&s));
}

pub fn solve_part2() {
    let m = parse_map();
    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 1, y: 3 },
        Slope { x: 1, y: 5 },
        Slope { x: 1, y: 7 },
        Slope { x: 2, y: 1 },
    ];

    let n: i64 = slopes.iter().map(|s| m.count_tree_for_slope(s)).product();

    println!("{}", n);
}
