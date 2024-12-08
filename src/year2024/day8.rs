use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: i32 = 0;
    let mut solution2: i32 = 0;

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in &lines {
        let line: Vec<u8> = line.as_bytes().to_vec();
        grid.push(line);
    }

    let w: i32 = grid[0].len() as i32;
    let h: i32 = grid.len() as i32;

    let mut pos: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = grid[y as usize][x as usize];
            if c != '.' as u8 {
                pos.entry(c).or_default().push((x, y));
            }
        }
    }
    let mut antinode = vec![vec![false; w as usize]; h as usize];
    for p in &pos {
        let (_, p) = p;
        if p.len() >= 2 {
            for (p1, p2) in p.iter().cartesian_product(p.iter()) {
                if p1 != p2 {
                    let (x1, y1) = p1;
                    let (x2, y2) = p2;
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let n1x = x2 + dx;
                    let n1y = y2 + dy;
                    let n2x = x1 - dx;
                    let n2y = y1 - dy;
                    if n1x >= 0 && n1x < w && n1y >= 0 && n1y < h {
                        antinode[n1y as usize][n1x as usize] = true;
                    }
                    if n2x >= 0 && n2x < w && n2y >= 0 && n2y < h {
                        antinode[n2y as usize][n2x as usize] = true;
                    }
                }
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            if antinode[y as usize][x as usize] {
                solution1 += 1;
            }
        }
    }

    let mut antinode = vec![vec![false; w as usize]; h as usize];
    for p in pos {
        let (_, p) = p;
        if p.len() >= 2 {
            for (p1, p2) in p.iter().cartesian_product(p.iter()) {
                if p1 != p2 {
                    let (x1, y1) = p1;
                    let (x2, y2) = p2;
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let mut n1x = *x2;
                    let mut n1y = *y2;
                    let mut n2x = *x1;
                    let mut n2y = *y1;
                    loop {
                        if n1x >= 0 && n1x < w && n1y >= 0 && n1y < h {
                            antinode[n1y as usize][n1x as usize] = true;
                        } else {
                            break;
                        }
                        n1x += dx;
                        n1y += dy;
                    }
                    loop {
                        if n2x >= 0 && n2x < w && n2y >= 0 && n2y < h {
                            antinode[n2y as usize][n2x as usize] = true;
                        } else {
                            break;
                        }
                        n2x -= dx;
                        n2y -= dy;
                    }
                }
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            if antinode[y as usize][x as usize] {
                solution2 += 1;
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
