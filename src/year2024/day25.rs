pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let solution2: u64 = 0;

    let mut i = lines.iter().peekable();
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();
    loop {
        if let Some(first) = i.peek() {
            let is_lock = *first == "#####";
            if is_lock {
                i.next();
            }
            let mut counts = vec![0; 5];
            for _ in 0..6 {
                let l = i.next().unwrap();
                for (i, c) in l.chars().enumerate() {
                    if c == '#' {
                        counts[i] += 1;
                    }
                }
            }
            if is_lock {
                locks.push(counts);
            } else {
                keys.push(counts);
                i.next();
            }
        } else {
            break;
        }
        i.next();
    }

    for lock in locks {
        for key in &keys {
            let mut too_high = false;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    too_high = true;
                    break;
                }
            }
            if !too_high {
                solution1 += 1;
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
