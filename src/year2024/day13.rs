use regex::Regex;

fn parse_pair(line: &String) -> (u128, u128) {
    let bre = Regex::new(r".*?(\d+).+?(\d+)").unwrap();
    let caps = bre.captures(line).unwrap();
    (
        caps[1].parse::<u128>().unwrap(),
        caps[2].parse::<u128>().unwrap(),
    )
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u128 = 0;
    let mut solution2: u128 = 0;

    let mut li = lines.iter();
    loop {
        if let Some(line) = li.next() {
            let (x1, y1) = parse_pair(line);
            let (x2, y2) = parse_pair(li.next().unwrap());
            let (mut px, mut py) = parse_pair(li.next().unwrap());
            li.next();

            // part one
            for b1 in 0..=100 {
                for b2 in 0..=100 {
                    if x1 * b1 + x2 * b2 == px && y1 * b1 + y2 * b2 == py {
                        let cost = 3 * b1 + b2;
                        solution1 += cost as u128;
                    }
                }
            }

            // part two
            px += 10000000000000;
            py += 10000000000000;
            let det: i128 = (x1 * y2) as i128 - (x2 * y1) as i128;
            if det != 0 {
                let num: i128 = (px * y2) as i128 - (x2 * py) as i128;
                if num.abs() % det.abs() == 0 {
                    let b1 = num / det;
                    let num: i128 = px as i128 - x1 as i128 * b1;
                    if num % x2 as i128 == 0 {
                        let b2 = num / x2 as i128;
                        let cost = 3 * b1 + b2;
                        solution2 += cost as u128;
                    }
                }
            }
        } else {
            break;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
