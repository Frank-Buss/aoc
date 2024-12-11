use num::bigint::BigInt;
use num::Integer;
use memoize::memoize;

#[memoize]
fn blink(count: i64, s: BigInt) -> u64 {
    if count == 0 {
        return 1;
    }
    let count = count - 1;
    if s == 0.into() {
        return blink(count, 1.into());
    } else {
        let d = s.to_string();
        if d.len().is_even() {
            return blink(count, d[0..d.len() / 2].parse().unwrap())
                + blink(count, d[d.len() / 2..].parse().unwrap());
        } else {
            return blink(count, s * 2024);
        }
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let mut solution2: u64 = 0;

    let mut stones: Vec<BigInt> = Vec::new();
    for s in lines[0].split_ascii_whitespace() {
        stones.push(s.parse::<BigInt>().unwrap());
    }
    for s in &stones {
        solution1 += blink(25, s.clone());
        solution2 += blink(75, s.clone());
    }

    (solution1.to_string(), solution2.to_string())
}
