use regex::Regex;

fn parse_pair(line: &String) -> (u32, u32) {
    let bre = Regex::new(r".*?(\d+).+?(\d+)").unwrap();
    let caps = bre.captures(line).unwrap();
    (
        caps[1].parse::<u32>().unwrap(),
        caps[2].parse::<u32>().unwrap(),
    )
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut li = lines.iter();
    let mut costs: Vec<u32> = Vec::new();
    loop {
        if let Some(line) = li.next() {
            let (b1x, b1y) = parse_pair(line);
            let (b2x, b2y) = parse_pair(li.next().unwrap());
            let (px, py) = parse_pair(li.next().unwrap());
            li.next();
            for b1 in 0..=100 {
                for b2 in 0..=100 {
                    if b1x * b1 + b2x * b2 == px && b1y * b1 + b2y * b2 == py {
                        costs.push(3 * b1 + b2);
                    }
                }
            }
        } else {
            break;
        }
    }

    let solution1: u32 = costs.iter().sum();

    (solution1.to_string(), "0".into())
}
