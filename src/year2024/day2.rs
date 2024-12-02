fn is_save(levels: &Vec<i32>) -> bool {
    let mut last: i32 = -1;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut count = 0;
    for level in levels {
        if last >= 0 {
            let d = last - level;
            if d < 0 {
                decrease_count += 1;
            }
            if d > 0 {
                increase_count += 1;
            }

            // not safe if not increasing or decreasing by 1, 2, or 3
            if !(1..=3).contains(&d.abs()) {
                return false;
            }
        }
        last = *level;
        count += 1;
    }

    // test if all pairs are increasing or decreasing
    count -= 1;
    increase_count == count || decrease_count == count
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    // count number of safe lines
    let mut solution1 = 0;
    for line in &lines {
        let levels = line.split_whitespace();
        let levels = levels.map(|x| x.parse::<i32>().unwrap()).collect();
        if is_save(&levels) {
            solution1 += 1;
        }
    }

    // allow one unsafe level
    let mut solution2 = 0;
    for line in &lines {
        let levels = line.split_whitespace();
        let levels = levels.map(|x| x.parse::<i32>().unwrap()).collect();
        if is_save(&levels) {
            solution2 += 1;
        } else {
            // brute force it by testing to remove each level
            for i in 0..levels.len() {
                let mut levels = levels.clone();
                levels.remove(i);
                if is_save(&levels) {
                    solution2 += 1;
                    break;
                }
            }
        }
    }

    // return solutions
    (solution1, solution2)
}
