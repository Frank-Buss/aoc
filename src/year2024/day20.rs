use std::hash::Hash;
use std::i32;

use pathfinding::prelude::dijkstra;

type Grid = Vec<Vec<char>>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn maybe_add(&self, grid: &Grid, result: &mut Vec<Point>, w: i32, h: i32, dx: i32, dy: i32) {
        let mut x = self.x;
        let mut y = self.y;
        x += dx;
        y += dy;
        if x >= 0 && x < w && y >= 0 && y < h {
            let c = grid[y as usize][x as usize];
            if c == '.' {
                result.push(Point { x, y });
            }
        }
    }

    fn successors(&self, grid: &Grid) -> Vec<(Point, usize)> {
        let w: i32 = grid[0].len() as i32;
        let h: i32 = grid.len() as i32;
        let mut result = Vec::new();
        self.maybe_add(grid, &mut result, w, h, -1, 0);
        self.maybe_add(grid, &mut result, w, h, 0, -1);
        self.maybe_add(grid, &mut result, w, h, 1, 0);
        self.maybe_add(grid, &mut result, w, h, 0, 1);
        result.into_iter().map(|p| (p, 1)).collect()
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut grid: Grid = Vec::new();

    let mut i = lines.iter();
    loop {
        if let Some(line) = i.next() {
            if line.len() == 0 {
                break;
            }
            let line: Vec<char> = line.chars().collect();
            grid.push(line);
        } else {
            break;
        }
    }

    let mut sx: i32 = 0;
    let mut sy: i32 = 0;
    let mut ex: i32 = 0;
    let mut ey: i32 = 0;
    let w: i32 = grid[0].len() as i32;
    let h: i32 = grid.len() as i32;
    for y in 0..h {
        for x in 0..w {
            match grid[y as usize][x as usize] {
                'S' => {
                    sx = x;
                    sy = y;
                    grid[y as usize][x as usize] = '.';
                }
                'E' => {
                    ex = x;
                    ey = y;
                    grid[y as usize][x as usize] = '.';
                }
                _ => {}
            }
        }
    }

    let start = Point { x: sx, y: sy };
    let goal = Point { x: ex, y: ey };

    let mut solution1 = 0;
    let no_cheat_len = dijkstra(&start, |p| p.successors(&grid), |p| *p == goal)
        .unwrap()
        .0
        .len();
    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == '#' {
                grid[y as usize][x as usize] = '.';
                let len = dijkstra(&start, |p| p.successors(&grid), |p| *p == goal)
                    .unwrap()
                    .0
                    .len();
                if len < no_cheat_len {
                    let d = no_cheat_len - len;
                    if d >= 100 {
                        solution1 += 1;
                    }
                }
                grid[y as usize][x as usize] = '#';
            }
        }
    }

    let solution2 = 0;

    (solution1.to_string(), solution2.to_string())
}
