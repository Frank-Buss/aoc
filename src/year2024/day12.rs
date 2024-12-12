use std::collections::HashSet;

use itertools::Itertools;

fn get(grid: &Vec<Vec<u8>>, w: i32, h: i32, x: i32, y: i32) -> u8 {
    if x < 0 || x >= (w as i32) || y < 0 || y >= (h as i32) {
        return 0;
    }
    grid[y as usize][x as usize]
}

fn fill(
    p: u8,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    grid: &Vec<Vec<u8>>,
    v: &mut Vec<Vec<bool>>,
) -> (u32, Vec<(i32, i32)>) {
    if v[y as usize][x as usize] {
        return (0, Vec::new());
    }
    v[y as usize][x as usize] = true;
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut area = 1;
    let mut border: Vec<(i32, i32)> = Vec::new();
    for (dx, dy) in dirs {
        let x = (x as i32) + dx;
        let y = (y as i32) + dy;
        if get(grid, w, h, x, y) == p {
            let (a, mut b) = fill(p, x, y, w, h, grid, v);
            area += a;
            border.append(&mut b);
        } else {
            border.push((x, y));
        }
    }
    (area, border)
}

fn count_contiguous(v: &mut Vec<i32>) -> u32 {
    v.sort();
    let mut x0 = -3;
    let mut count = 0;
    for x in v.clone() {
        let dx = x - x0;
        if dx > 1 {
            count += 1;
        }
        x0 = x;
    }
    println!("{:?} = {}", v, count);
    count
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u32 = 0;
    let mut solution2: u32 = 0;

    let grid: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|line| line.bytes().collect())
        .collect();

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut visited: HashSet<u8> = HashSet::new();
    let mut v = vec![vec![false; w as usize]; h as usize];
    for y in 0..h {
        for x in 0..w {
            let p = grid[y as usize][x as usize];
            if !visited.contains(&p) || true {
                visited.insert(p);
                let (area, mut border) = fill(p, x, y, w, h, &grid, &mut v);
                solution1 += area * (border.len() as u32);

                if border.len() > 0 {
                    // group by same x coordinate
                    let mut ys = (&border).into_iter().into_group_map_by(|x| x.0);
                    println!("{:?}", border);
                    //println!("{:?}", ys);

                    // extract all y coordinates of each group
                    let ys: Vec<Vec<i32>> = ys
                        .iter()
                        .map(|(_, ys)| ys.iter().map(|(_, y)| *y).collect())
                        .collect();

                    // find contiguous y coordinates for each x coordinate
                    //println!("ys: {:?}", ys);
                    let ys = ys
                        .into_iter()
                        .map(|mut a| count_contiguous(&mut a))
                        .into_iter()
                        .reduce(|a, b| a + b)
                        .unwrap();

                    //println!("{:?}", c);

                    // group by same y coordinate
                    let mut xs = (&border).into_iter().into_group_map_by(|x| x.1);
                    //println!("{:?}", border);
                    //println!("{:?}", ys);

                    // extract all x coordinates of each group
                    let xs: Vec<Vec<i32>> = xs
                        .iter()
                        .map(|(_, xs)| xs.iter().map(|(x, _)| *x).collect())
                        .collect();

                    // find contiguous y coordinates for each x coordinate
                    //println!("ys: {:?}", ys);
                    let xs = xs
                        .into_iter()
                        .map(|mut a| count_contiguous(&mut a))
                        .into_iter()
                        .reduce(|a, b| a + b)
                        .unwrap();

                    println!("{} {}", p as char, xs+ys);
                }
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
