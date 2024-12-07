use std::collections::HashMap;

pub fn solve(lines: Vec<String>) -> (String, String) {
    // split each line and store it in 2 int vectors
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<i32>().unwrap());
        right.push(parts.next().unwrap().parse::<i32>().unwrap());
    }

    // sort vectors in-place
    left.sort();
    right.sort();

    // sum differences
    let mut solution1 = 0;
    for i in 0..left.len() {
        solution1 += (left[i] - right[i]).abs();
    }

    // create histogram
    let mut counts = HashMap::<i32, i32>::new();
    for i in right {
        *counts.entry(i).or_default() += 1;
    }

    // sum each left entry by number of occurences in right vector
    let mut solution2 = 0;
    for i in left {
        solution2 += *counts.entry(i).or_default() * i;
    }

    // return solutions
    (solution1.to_string(), solution2.to_string())
}
