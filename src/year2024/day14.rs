use regex::Regex;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let mut solution2: u64 = 0;

    let r = Regex::new(r".+?(\d+).+?(\d+).+?(-?\d+).+?(-?\d+)").unwrap();
    let mut bots: Vec<(i64, i64, i64, i64)> = Vec::new();
    for line in lines {
        if line.len() > 0 {
            let caps = r.captures(&line).unwrap();
            let px = caps[1].parse::<i64>().unwrap();
            let py = caps[2].parse::<i64>().unwrap();
            let vx = caps[3].parse::<i64>().unwrap();
            let vy = caps[4].parse::<i64>().unwrap();
            bots.push((px, py, vx, vy));
        }
    }

    let w = 101;
    let h = 103;
    let mut grid = vec![vec![0; w as usize]; h as usize];
    for i in 1.. {
        for bot in &mut bots {
            bot.0 += bot.2;
            bot.1 += bot.3;
            if bot.0 < 0 {
                bot.0 += w;
            }
            if bot.0 >= w {
                bot.0 -= w;
            }
            if bot.1 < 0 {
                bot.1 += h;
            }
            if bot.1 >= h {
                bot.1 -= h;
            }
        }

        // part 1
        if i == 100 {
            let mut q0 = 0;
            let mut q1 = 0;
            let mut q2 = 0;
            let mut q3 = 0;
            for bot in &bots {
                if bot.0 < w / 2 && bot.1 < h / 2 {
                    q0 += 1;
                }
                if bot.0 > w / 2 && bot.1 < h / 2 {
                    q1 += 1;
                }
                if bot.0 < w / 2 && bot.1 > h / 2 {
                    q2 += 1;
                }
                if bot.0 > w / 2 && bot.1 > h / 2 {
                    q3 += 1;
                }
            }
            solution1 += q0 * q1 * q2 * q3;
        }

        // for part 2, assume it is solved, when all bots are on their own fields
        grid.iter_mut().for_each(|row| row.fill(0));
        let mut double = false;
        for bot in &bots {
            grid[bot.1 as usize][bot.0 as usize] += 1;
            if grid[bot.1 as usize][bot.0 as usize] > 1 {
                double = true;
                break;
            }
        }
        if !double {
            for y in 0..h {
                for x in 0..w {
                    let mut found = false;
                    for bot in &bots {
                        if bot.0 == x && bot.1 == y {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        print!("x");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            solution2 = i;
            break;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
