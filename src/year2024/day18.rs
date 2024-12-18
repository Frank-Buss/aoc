use std::collections::HashMap;

use std::hash::Hash;
use std::i32;

use regex::Regex;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn read_ints(line: &String) -> Vec<u64> {
    let r = r"(\d+)";
    let mut ints: Vec<u64> = Vec::new();
    let r = Regex::new(&r).unwrap();
    for cap in r.find_iter(&line) {
        ints.push(cap.as_str().parse::<u64>().unwrap());
    }
    ints
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let w = 71;
    let h = 71;
    let mut org_grid = vec![vec![false; w as usize]; h as usize];
    let solution2;

    let mut grid = org_grid.clone();
    let mut line_iter = lines.iter();
    for _ in 0..1024 {
        let line = line_iter.next().unwrap();
        if line.len() == 0 {
            break;
        }
        let p = read_ints(line);
        grid[p[1] as usize][p[0] as usize] = true;
    }

    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut rs: Vec<Point> = Vec::new();
    let start = Point { x: 0, y: 0 };
    let mut visited: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut path: Vec<Point> = Vec::new();
    path.push(start);
    visited.insert(start, path);
    rs.push(start);
    let mut wins: Vec<Vec<Point>> = Vec::new();
    let ex = w - 1;
    let ey = h - 1;
    loop {
        if rs.len() == 0 {
            break;
        }
        let p = rs.pop().unwrap();
        for dir in 0..dirs.len() {
            let (dx, dy) = dirs[dir];
            let prev_path = visited.get(&p).unwrap();
            let x = (p.x as i32) + dx;
            let y = (p.y as i32) + dy;
            if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
                let x = x as usize;
                let y = y as usize;
                if !grid[y][x] {
                    let next = Point { x, y };
                    let mut path = prev_path.clone();
                    path.push(next);
                    if !visited.contains_key(&next)
                        || visited.get(&next).unwrap().len() > path.len()
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
    }

    // find lowest cost path
    let mut min_cost = usize::MAX;
    for path in &wins {
        if path.len() < min_cost {
            min_cost = path.len();
        }
    }

    let solution1 = min_cost - 1;

    // try flood fill for part 2
    loop {
        let mut last = Point { x: 0, y: 0 };
        let line = line_iter.next().unwrap();
        let p = read_ints(line);
        org_grid[p[1] as usize][p[0] as usize] = true;
        last.x = p[0] as usize;
        last.y = p[1] as usize;
        grid = org_grid.clone();

        let mut rs: Vec<Point> = Vec::new();
        rs.push(Point { x: 0, y: 0 });
        grid[0][0] = true;
        let mut exit = false;
        let ex = w - 1;
        let ey = h - 1;
        'outer: loop {
            if rs.len() == 0 {
                break;
            }
            let p = rs.pop().unwrap();
            for dir in 0..dirs.len() {
                let (dx, dy) = dirs[dir];
                let x = (p.x as i32) + dx;
                let y = (p.y as i32) + dy;
                if x >= 0 && y >= 0 && x < w as i32 && y < h as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    if !grid[y][x] {
                        grid[y][x] = true;
                        if x == ex && y == ey {
                            exit = true;
                            break 'outer;
                        }
                        rs.push(Point { x, y });
                    }
                }
            }
        }

        // no path to exit
        if !exit {
            solution2 = format!("{},{}", last.x, last.y);
            break;
        }
    }

    (solution1.to_string(), solution2)
}
