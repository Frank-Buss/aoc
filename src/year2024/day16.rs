use std::collections::HashMap;

use std::hash::Hash;
use std::i32;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct R {
    pos: Point,
    dir: usize,
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut i = lines.iter();
    loop {
        let line = i.next().unwrap().trim();
        if line.len() == 0 {
            break;
        }
        let line: Vec<char> = line.chars().collect();
        grid.push(line);
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
    let dir = 0;
    let mut rs: Vec<R> = Vec::new();
    let start = R {
        pos: Point { x: sx, y: sy },
        dir,
    };
    let mut visited: HashMap<R, Vec<Point>> = HashMap::new();
    rs.push(start);
    let mut wins: Vec<Vec<Point>> = Vec::new();
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
            path.push(r.pos);
            match grid[y][x] {
                '.' | 'E' => {
                    let next = R {
                        pos: Point { x, y },
                        dir,
                    };
                    if visited.contains_key(&next) {
                        // if already visited, set new path, if shorter
                        let path2 = visited.get(&next).unwrap();
                        if path2.len() < path.len() {
                            visited.insert(r, path2.clone());
                        }
                    } else {
                        // if not visited, mark it as visited with the current path
                        visited.insert(next, path.clone());

                        if x == ex && y == ey {
                            // store winning position
                            wins.push(path.clone());
                        } else {
                            // search next
                            rs.push(next.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // find lowest cost path
    let mut min_cost = i32::MAX;
    for path in wins {
        let p0 = path[0];
        let mut dir0 = 0;
        let mut cost = 0;
        for p in path[1..].iter() {
            let dx = (p0.x as i32)-(p.x as i32);
            let dy = (p0.x as i32)-(p.x as i32);
            let dir = dirs.iter().position(|&d| d == (dx,dy)).unwrap();
            if dir != dir0 {
                cost += 1000;
                dir0 = dir;
            }
            cost += 1;
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    let solution1 = min_cost;

    (solution1.to_string(), solution1.to_string())
}
