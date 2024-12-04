fn get(grid: &Vec<Vec<u8>>, width: i32, height: i32, x: i32, y: i32) -> char {
    if x < 0 || x >= width || y < 0 || y >= height {
        return '.';
    }
    grid[y as usize][x as usize] as char
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut solution1 = 0;
    let mut solution2 = 0;
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in &lines {
        grid.push(line.as_bytes().to_vec());
    }
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let test = "XMAS".to_string();
    let test = test.as_bytes().to_vec();
    for x in 0..width {
        for y in 0..height {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let mut found = 0;
                    for d in 0..test.len() {
                        let d: i32 = d as i32;
                        if get(&grid, width, height, x + d * dx, y + d * dy)
                            == test[d as usize] as char
                        {
                            found += 1;
                        }
                        if found == test.len() {
                            solution1 += 1;
                        }
                    }
                }
            }
            if get(&grid, width, height, x, y) == 'A'
                && ((get(&grid, width, height, x - 1, y - 1) == 'M'
                    && get(&grid, width, height, x + 1, y + 1) == 'S')
                    || (get(&grid, width, height, x - 1, y - 1) == 'S'
                        && get(&grid, width, height, x + 1, y + 1) == 'M'))
                && ((get(&grid, width, height, x + 1, y - 1) == 'M'
                    && get(&grid, width, height, x - 1, y + 1) == 'S')
                    || (get(&grid, width, height, x + 1, y - 1) == 'S'
                        && get(&grid, width, height, x - 1, y + 1) == 'M'))
            {
                solution2 += 1;
            }
        }
    }

    // return solutions
    (solution1, solution2)
}
