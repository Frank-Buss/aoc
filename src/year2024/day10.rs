fn get(grid: &Vec<Vec<u32>>, width: i32, height: i32, x: i32, y: i32) -> u32 {
    if x < 0 || x >= width || y < 0 || y >= height {
        return 0;
    }
    grid[y as usize][x as usize]
}

fn search(
    level: u32,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
    visited: &mut Option<&mut Vec<Vec<bool>>>,
    grid: &Vec<Vec<u32>>,
) -> i32 {
    if let Some(v) = visited {
        v[y as usize][x as usize] = true;
    }
    if level == 9 {
        // top reached
        1
    } else {
        // check if climbing is possible
        let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut sum = 0;
        for dir in dirs {
            let (dx, dy) = dir;
            let x = x + dx;
            let y = y + dy;

            // bit tricky here: when a next level is possible, it is within the grid,
            // so no bounds check for visited required
            let next = get(&grid, w, h, x, y);
            if next == level + 1 {
                if let Some(v) = &*visited {
                    if v[y as usize][x as usize] {
                        continue;
                    }
                }
                sum += search(next, w, h, x, y, visited, &grid);
            }
        }
        sum
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: i32 = 0;
    let mut solution2: i32 = 0;

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in &lines {
        let line: Vec<char> = line.chars().collect();
        grid.push(line.iter().map(|x| x.to_digit(10).unwrap()).collect());
    }

    // test all possible trail starts
    let w: i32 = grid[0].len() as i32;
    let h: i32 = grid.len() as i32;
    for y in 0..h {
        for x in 0..w {
            let start = grid[y as usize][x as usize];
            if start == 0 {
                let mut visited = vec![vec![false; w as usize]; h as usize];
                solution1 += search(start, w, h, x, y, &mut Some(&mut visited), &grid);
                solution2 += search(start, w, h, x, y, &mut None, &grid);
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
