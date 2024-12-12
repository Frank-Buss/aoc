pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let mut solution2: u64 = 0;

    for line in lines {
        if line.len() > 0 {
            solution1 += 1;
            solution2 += 1;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
