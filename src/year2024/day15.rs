pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: i32 = 0;
    let mut solution2: i32 = 0;

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

    let mut grid2: Vec<Vec<char>> = vec![vec!['.'; 2 * w as usize]; h as usize];
    for y in 0..h {
        for x in 0..w {
            let x2 = 2 * x as usize;
            let y2 = y as usize;
            match grid[y as usize][x as usize] {
                '#' => {
                    grid2[y2][x2] = '#';
                    grid2[y2][x2 + 1] = '#';
                }
                'O' => {
                    grid2[y2][x2] = '[';
                    grid2[y2][x2 + 1] = ']';
                }
                '@' => {
                    grid2[y2][x2] = '@';
                }
                _ => {}
            }
        }
    }

    let mut moves: Vec<char> = Vec::new();
    loop {
        let line = i.next();
        if let Some(line) = line {
            let mut line: Vec<char> = line.chars().collect();
            moves.append(&mut line);
        } else {
            break;
        }
    }

    let mut x0 = 0;
    let mut y0 = 0;
    'start: for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == '@' {
                x0 = x;
                y0 = y;
                break 'start;
            }
        }
    }
    let xs = x0;
    let ys = y0;

    for m in &moves {
        let mut dx = 0;
        let mut dy = 0;
        match m {
            '>' => {
                dx = 1;
                dy = 0;
            }
            '^' => {
                dx = 0;
                dy = -1;
            }
            'v' => {
                dx = 0;
                dy = 1;
            }
            '<' => {
                dx = -1;
                dy = 0;
            }
            _ => {}
        }
        let x1 = x0 + dx;
        let y1 = y0 + dy;
        match grid[y1 as usize][x1 as usize] {
            '.' => {
                grid[y0 as usize][x0 as usize] = '.';
                grid[y1 as usize][x1 as usize] = '@';
                x0 = x1;
                y0 = y1;
            }
            'O' => {
                let mut x2 = x1;
                let mut y2 = y1;
                loop {
                    x2 += dx;
                    y2 += dy;
                    match grid[y2 as usize][x2 as usize] {
                        '.' => {
                            grid[y2 as usize][x2 as usize] = 'O';
                            grid[y0 as usize][x0 as usize] = '.';
                            grid[y1 as usize][x1 as usize] = '@';
                            x0 = x1;
                            y0 = y1;
                            break;
                        }
                        '#' => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == 'O' {
                solution1 += 100 * y + x;
            }
        }
    }

    // part 2
    x0 = 2 * xs;
    y0 = ys;
    grid = grid2.clone();
    let w = 2 * w;
    for m in moves {
        println!("move: {}", m);
        let mut dx = 0;
        let mut dy = 0;
        match m {
            '>' => {
                dx = 1;
                dy = 0;
            }
            '^' => {
                dx = 0;
                dy = -1;
            }
            'v' => {
                dx = 0;
                dy = 1;
            }
            '<' => {
                dx = -1;
                dy = 0;
            }
            _ => {}
        }
        let x1 = x0 + dx;
        let y1 = y0 + dy;
        match grid[y1 as usize][x1 as usize] {
            '.' => {
                grid[y0 as usize][x0 as usize] = '.';
                grid[y1 as usize][x1 as usize] = '@';
                x0 = x1;
                y0 = y1;
            }
            ']' | '[' => {
                if dy == 0 {
                    // horizontal box movement
                    let mut x2 = x1;
                    loop {
                        x2 += dx;
                        match grid[y0 as usize][x2 as usize] {
                            '.' => {
                                loop {
                                    grid[y0 as usize][x2 as usize] =
                                        grid[y0 as usize][(x2 - dx) as usize];
                                    x2 -= dx;
                                    if x2 == x1 {
                                        break;
                                    }
                                }
                                grid[y0 as usize][x0 as usize] = '.';
                                grid[y0 as usize][x1 as usize] = '@';
                                x0 = x1;
                                break;
                            }
                            '#' => {
                                break;
                            }
                            _ => {}
                        }
                    }
                } else {
                    // vertical box movement
                    let mut boxline: Vec<(i32, i32)> = Vec::new();
                    boxline.push((x1, y1));
                    if grid[y1 as usize][x1 as usize] == '[' {
                        boxline.push((x1 + 1, y1));
                    } else {
                        boxline.push((x1 - 1, y1));
                    }

                    // search in dy direction, until wall, or all free
                    let mut boxes: Vec<Vec<(i32, i32)>> = Vec::new();
                    boxes.push(boxline.clone());
                    loop {
                        let mut nextline: Vec<(i32, i32)> = Vec::new();
                        let mut wall = false;
                        for (x, y) in &boxline {
                            let x = *x;
                            let y = *y + dy;
                            // this pushes twice, e.g. if [] on top of [], but doesn't matter
                            match grid[y as usize][x as usize] {
                                '[' => {
                                    nextline.push((x, y));
                                    nextline.push((x + 1, y));
                                }
                                ']' => {
                                    nextline.push((x, y));
                                    nextline.push((x - 1, y));
                                }
                                '#' => {
                                    wall = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        if wall {
                            break;
                        }
                        if nextline.len() == 0 {
                            // only free positions, move everything
                            for line in boxes.iter().rev() {
                                for (x, y) in line {
                                    let x = *x;
                                    let y = *y;
                                    grid[(y + dy) as usize][x as usize] =
                                        grid[y as usize][x as usize];

                                        // fill moved field with empty, can be overwritten for the next line
                                        grid[y as usize][x as usize]='.';
                                }
                            }

                            // move robot
                            grid[y0 as usize][x0 as usize] = '.';
                            grid[y1 as usize][x0 as usize] = '@';
                            y0 = y1;
                            break;
                        } else {
                            // there were more boxes, check next line
                            boxes.push(nextline.clone());
                            boxline = nextline.clone();
                        }
                    }
                }
            }
            _ => {}
        }
        for y in 0..h {
            for x in 0..w {
                print!("{}", grid[y as usize][x as usize]);
            }
            println!("");
        }
    }

    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == '[' {
                solution2 += 100 * y + x;
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
