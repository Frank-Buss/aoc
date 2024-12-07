fn run(
    x1: i32,
    y1: i32,
    w: i32,
    h: i32,
    dir: usize,
    grid: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<char>>,
    history: &mut Vec<Vec<u8>>,
) -> bool {
    for y in 0..h {
        for x in 0..w {
            history[y as usize][x as usize] = 0;
        }
    }
    let dirs: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir = dir;
    let mut x1 = x1;
    let mut y1 = y1;
    loop {
        let (dx, dy) = dirs[dir];
        visited[y1 as usize][x1 as usize] = 'X';
        let bit = 1 << dir;
        if history[y1 as usize][x1 as usize] & bit > 0 {
            return true;
        }
        history[y1 as usize][x1 as usize] |= bit;

        let x2: i32 = x1 + dx;
        let y2: i32 = y1 + dy;
        if x2 < 0 || x2 >= w || y2 < 0 || y2 >= h {
            return false;
        }
        if grid[y2 as usize][x2 as usize] == '#' {
            dir += 1;
            if dir == 4 {
                dir = 0;
            }
        } else {
            x1 = x2;
            y1 = y2;
        }
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut dir = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in &lines {
        let line: Vec<char> = line.chars().collect();
        grid.push(line.clone());
    }

    let mut visited = grid.clone();
    let w: i32 = grid[0].len() as i32;
    let h: i32 = grid.len() as i32;
    let mut x1: i32 = 0;
    let mut y1: i32 = 0;
    for x in 0..w {
        for y in 0..h {
            let c = grid[y as usize][x as usize];
            let d = match c {
                '>' => 0,
                'v' => 1,
                '<' => 2,
                '^' => 3,
                _ => 4,
            };
            if d != 4 {
                dir = d;
                x1 = x;
                y1 = y;
            }
        }
    }

    let mut history = vec![vec![0; w as usize]; h as usize];
    run(x1, y1, w, h, dir, &mut grid, &mut visited, &mut history);

    let mut solution1 = 0;
    for x in 0..w {
        for y in 0..h {
            let c = visited[y as usize][x as usize];
            if c == 'X' {
                solution1 += 1;
            }
        }
    }

    let mut solution2 = 0;
    for x in 0..w {
        for y in 0..h {
            let c = grid[y as usize][x as usize];
            if c == '.' {
                grid[y as usize][x as usize] = '#';
                let looped = run(x1, y1, w, h, dir, &mut grid, &mut visited, &mut history);
                if looped {
                    solution2 += 1;
                }
                grid[y as usize][x as usize] = '.';
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
