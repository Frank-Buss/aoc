use itertools::Itertools;
use std::collections::HashMap;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: i64 = 0;
    let mut solution2: i64 = 0;

    let mut disk: Vec<i32> = Vec::new();
    let mut file = true;
    let mut id = 0;
    for c in lines[0].chars() {
        let count = c.to_digit(10).unwrap();
        for i in 0..count {
            if file {
                disk.push(id);
            } else {
                disk.push(-1);
            }
        }
        if file {
            id += 1;
        }
        file = !file;
    }

    for i in 0..disk.len() {
        if disk[i] == -1 {
            for j in (i..disk.len()).rev() {
                if disk[j] != -1 {
                    disk[i] = disk[j];
                    disk[j] = -1;
                    break;
                }
            }
        }
    }

    for i in 0..disk.len() {
        let id = disk[i];
        if id >= 0 {
            solution1 += ((i as i32) * disk[i]) as i64;
        }
    }

    let mut disk: Vec<(i32, i32)> = Vec::new();
    let mut file = true;
    let mut id = 0;
    for c in lines[0].chars() {
        let count = c.to_digit(10).unwrap() as i32;
        disk.push((count, if file { id } else { -1 }));
        if file {
            id += 1;
        }
        file = !file;
    }

    let mut i = disk.len() - 1;
    loop {
        let (count, id) = disk[i];
        if id >= 0 {
            for j in 0..disk.len() {
                if j >= i {
                    break;
                }
                let (count2, id2) = disk[j];
                if id2 < 0 && count2 >= count {
                    disk[i] = (count, -1);
                    disk[j] = (count, id);
                    let rest = count2 - count;
                    if rest > 0 {
                        disk.insert(j + 1, (rest, -1));
                        i += 1;
                    }
                    break;
                }
            }
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }

    let mut pos = 0;
    for i in 0..disk.len() {
        let (count, id) = disk[i];
        for j in 0..count {
            if id >= 0 {
                solution2 += (pos * id) as i64;
            }
            pos += 1;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
