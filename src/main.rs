use std::fs;
mod year2024;

fn main() {
    // read input
    let file = fs::read_to_string("src/year2024/day1.txt").unwrap();
    let lines = file.lines().collect();

    // calculate solution
    let (solution1, solution2) = year2024::day1::solve(lines);

    // show it
    println!("solution 1: {}", solution1);
    println!("solution 2: {}", solution2);
}
