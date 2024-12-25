use std::collections::HashMap;

use itertools::Itertools;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let solution2: u64 = 0;

    let mut i = lines.iter();
    let mut wires: HashMap<String, u8> = HashMap::new();

    loop {
        let line = i.next().unwrap();
        if line.len() > 0 {
            let mut parts = line.split_whitespace();

            let w = parts.next().unwrap().to_string();
            let w = w[0..w.len() - 1].to_string();
            let v = parts.next().unwrap().parse::<u8>().unwrap();
            wires.insert(w, v);
        } else {
            break;
        }
    }

    let mut w1s: Vec<String> = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    let mut w2s: Vec<String> = Vec::new();
    let mut w3s: Vec<String> = Vec::new();

    loop {
        let line = i.next();
        if let Some(line) = line {
            let mut parts = line.split_whitespace();

            let w1 = parts.next().unwrap().to_string();
            let op = parts.next().unwrap().to_string();
            let w2 = parts.next().unwrap().to_string();
            parts.next();
            let w3 = parts.next().unwrap().to_string();

            w1s.push(w1.clone());
            ops.push(op);
            w2s.push(w2.clone());
            w3s.push(w3.clone());

            add_undefined(&mut wires, w1);
            add_undefined(&mut wires, w2);
            add_undefined(&mut wires, w3);
        } else {
            break;
        }
    }

    loop {
        let mut undefined = false;
        for i in 0..w1s.len() {
            let w1name = &w1s[i];
            let w2name = &w2s[i];
            let w1 = wires.get(w1name).unwrap();
            let op = &ops[i];
            let w2 = wires.get(w2name).unwrap();
            if *w1 != 2 && *w2 != 2 {
                let v = match op.as_str() {
                    "AND" => w1 & w2,
                    "OR" => w1 | w2,
                    "XOR" => w1 ^ w2,
                    _ => 0,
                };
                *wires.entry(w3s[i].clone()).or_default() = v;
            } else {
                undefined = true;
            }
        }
        if !undefined {
            break;
        }
    }

    for w in wires.keys().sorted().rev() {
        if w.starts_with("z") {
            let v = *wires.get(w).unwrap();
            solution1 <<= 1;
            if v > 0 {
                solution1 |= 1;
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}

fn add_undefined(wires: &mut HashMap<String, u8>, w: String) {
    if !wires.contains_key(&w) {
        wires.insert(w, 2);
    }
}
