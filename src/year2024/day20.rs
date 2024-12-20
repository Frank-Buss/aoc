use std::collections::HashMap;

use std::hash::Hash;
use std::i32;

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Path {
    p: Vec<Point>,
}

fn find_paths(
    grid: &Vec<Vec<char>>,
    sx: usize,
    sy: usize,
    ex: usize,
    ey: usize,
) -> Vec<Path> {
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut rs: Vec<Point> = Vec::new();
    let start = Point { x: sx, y: sy };
    let mut visited: HashMap<Point, Path> = HashMap::new();
    let mut path = Path { p: Vec::new() };
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
            let x = (r.x as i32) + dx;
            let y = (r.y as i32) + dy;
            let x = x as usize;
            let y = y as usize;
            let mut path = prev_path.clone();
            let g = grid[y][x];
            if g == '.' {
                let next = Point { x, y };
                path.p.push(next);
                if !visited.contains_key(&next)
                    || visited.get(&next).unwrap().p.len() >= path.p.len()
                {
                    visited.insert(next, path.clone());
                    if x == ex && y == ey {
                        wins.push(path.clone());
                    } else {
                        rs.push(next.clone());
                    }
                }
            }
        }
    }
    wins
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
                    grid[y as usize][x as usize] = '.';
                }
                'E' => {
                    ex = x as usize;
                    ey = y as usize;
                    grid[y as usize][x as usize] = '.';
                }
                _ => {}
            }
        }
    }

    let wins = find_paths(&grid, sx, sy, ex, ey);
    let mut wins_cheated = Vec::new();
    println!("w: {}, h: {}", w, h);
    for y in 0..h {
        for x in 0..w {
            let x = x as usize;
            let y = y as usize;
            if x == 0 || y == 0 || x==(w-1) as usize || y == (h-1) as usize {
                continue;
            }
            if grid[y][x] == '#' {
                println!("x: {}, y: {}", x, y);
                grid[y][x] = '.';
                let mut paths = find_paths(&grid, sx, sy, ex, ey);
                grid[y][x] = '#';
                wins_cheated.append(&mut paths);
            }
        }
    }

    // find lowest cost path with no cheats
    let mut min_cost = usize::MAX;
    for path in &wins {
        if path.p.len() < min_cost {
            min_cost = path.p.len();
        }
    }

    let best = min_cost;

    // find all paths which are better with cheating
    let mut count: HashMap<usize, usize> = HashMap::new();
    for path in &wins_cheated {
        if path.p.len() < best {
            let d = best - path.p.len();
            *count.entry(d).or_default() += 1;
        }
    }

    let mut solution1 = 0;
    for c in count.keys().sorted() {
        let ps = count.get(c).unwrap();
        println!(
            "- There are {} cheats that save {} picoseconds.",
            ps,
            c
        );
        if *ps >= 100 {
            solution1 += c;
        }
    }

    let solution2 = 0;

    (solution1.to_string(), solution2.to_string())
}
