use std::collections::{HashMap, HashSet};

use std::hash::Hash;
use std::i32;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct R {
    pos: Point,
    dir: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Path {
    p: Vec<R>,
    cost: usize,
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut grid: Vec<Vec<char>> = Vec::new();

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
    let w: i32 = grid[0].len() as i32;
    let h: i32 = grid.len() as i32;

    let mut sx: usize = 0;
    let mut sy: usize = 0;
    let mut ex: usize = 0;
    let mut ey: usize = 0;
    for y in 0..h {
        for x in 0..w {
            match grid[y as usize][x as usize] {
                'S' => {
                    sx = x as usize;
                    sy = y as usize;
                }
                'E' => {
                    ex = x as usize;
                    ey = y as usize;
                }
                _ => {}
            }
        }
    }

    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let dirs_char = b">v<^";
    let mut rs: Vec<R> = Vec::new();
    let start = R {
        pos: Point { x: sx, y: sy },
        dir: 0,
    };
    let mut visited: HashMap<R, Path> = HashMap::new();
    let mut path = Path {
        p: Vec::new(),
        cost: 0,
    };
    path.p.push(start);
    visited.insert(start, path);
    rs.push(start);
    let mut wins: Vec<Path> = Vec::new();
    loop {
        if rs.len() == 0 {
            break;
        }
        let r = rs.pop().unwrap();
        for dir in 0..dirs.len() {
            let (dx, dy) = dirs[dir];
            let prev_path = visited.get(&r).unwrap();
            let x = ((r.pos.x as i32) + dx) as usize;
            let y = ((r.pos.y as i32) + dy) as usize;
            let mut path = prev_path.clone();
            match grid[y][x] {
                '.' | 'E' => {
                    let next = R {
                        pos: Point { x, y },
                        dir,
                    };
                    let mut cost = prev_path.cost + 1;
                    let last_dir = prev_path.p.last().unwrap().dir;
                    if dir != last_dir {
                        cost += 1000;
                        if dir.abs_diff(last_dir) == 2 {
                            // 180°
                            cost += 1000;
                        }
                    }
                    path.p.push(next);
                    path.cost = cost;
                    if !visited.contains_key(&next) || visited.get(&next).unwrap().cost >= path.cost {
                        visited.insert(next, path.clone());
                        if x == ex && y == ey {
                            wins.push(path.clone());
                        } else {
                            rs.push(next.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // find lowest cost path
    let mut min_cost = usize::MAX;
    let mut win = wins[0].clone();
    for path in &wins {
        if path.cost < min_cost {
            min_cost = path.cost;
            win = path.clone();
        }
    }
    for r in win.p {
        grid[r.pos.y][r.pos.x] = dirs_char[r.dir] as char;
    }

    let solution1 = min_cost;

    let mut visited:HashSet<Point> = HashSet::new();
    for path in wins {
        if path.cost == solution1 {
            for p in path.p {
                visited.insert(p.pos);
                grid[p.pos.y][p.pos.x] = 'O';
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            print!("{}", grid[y as usize][x as usize]);
        }
        println!("");
    }

    let solution2 = visited.len();

    (solution1.to_string(), solution2.to_string())
}
