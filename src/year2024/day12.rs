use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Border {
    pos: Point,
    dir: Point,
}

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
) -> (u32, Vec<Border>) {
    if v[y as usize][x as usize] {
        return (0, Vec::new());
    }
    v[y as usize][x as usize] = true;
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut area = 1;
    let mut borders: Vec<Border> = Vec::new();
    for (dx, dy) in dirs {
        let x2 = (x as i32) + dx;
        let y2 = (y as i32) + dy;
        if get(grid, w, h, x2, y2) == p {
            let (a, mut b) = fill(p, x2, y2, w, h, grid, v);
            area += a;
            borders.append(&mut b);
        } else {
            borders.push(Border {
                pos: Point { x, y },
                dir: Point { x: dx, y: dy },
            });
        }
    }
    (area, borders)
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
    let mut v = vec![vec![false; w as usize]; h as usize];
    for y in 0..h {
        for x in 0..w {
            let p = grid[y as usize][x as usize];
            let (area, borders) = fill(p, x, y, w, h, &grid, &mut v);
            solution1 += area * (borders.len() as u32);

            // count all vertical fences
            let vertical_fences = borders
                .clone()
                .into_iter()
                // group by same x coordinate
                .into_group_map_by(|b| b.pos.x)
                .into_values()
                .map(|group| {
                    group
                        .into_iter()
                        // filter only vertical fences, left or right side of the center
                        .filter(|b| b.dir.x != 0)
                        // group by direction, to count all left contiguous and all right continguous borders
                        .into_group_map_by(|b| b.dir)
                        .into_values()
                        // sort all y coordinates, pair it, and then count gaps
                        // the number of contiguous borders is one more (e.g. no gap, it is one group)
                        .map(|y_group| {
                            1 + y_group
                                .into_iter()
                                .map(|b| b.pos.y)
                                .sorted()
                                .tuple_windows()
                                .filter(|(a, b)| (b - a).abs() > 1)
                                .count() as u32
                        })
                        .sum::<u32>()
                })
                .sum::<u32>();

            // count all horizontal fences
            let horizontal_fences = borders
                .clone()
                .into_iter()
                .into_group_map_by(|b| b.pos.y) // group by y coordinate
                .into_values()
                .map(|group| {
                    group
                        .into_iter()
                        .filter(|b| b.dir.y != 0) // only horizontal fences
                        .into_group_map_by(|b| b.dir) // group by direction
                        .into_values()
                        .map(|x_group| {
                            1 + x_group
                                .into_iter()
                                .map(|b| b.pos.x)
                                .sorted()
                                .tuple_windows()
                                .filter(|(a, b)| (b - a).abs() > 1)
                                .count() as u32
                        })
                        .sum::<u32>() // sum the gaps for each direction at this y coordinate
                })
                .sum::<u32>(); // sum across all y coordinates

            let fences = vertical_fences + horizontal_fences;
            solution2 += area * fences;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
