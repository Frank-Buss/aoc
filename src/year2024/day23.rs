use std::collections::{HashMap, HashSet};

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;

    // split each line and store it in 2 int vectors
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    for line in lines {
        let mut parts = line.split("-");
        lefts.push(parts.next().unwrap().to_string());
        rights.push(parts.next().unwrap().to_string());
    }

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for i in 0..lefts.len() {
        let left = lefts[i].clone();
        let right = rights[i].clone();
        connections
            .entry(left.clone())
            .or_default()
            .push(right.clone());
        connections.entry(right.clone()).or_default().push(left);
    }

    lefts.append(&mut rights);
    let comps = lefts;

    let mut visited: HashSet<String> = HashSet::new();
    for c in &comps {
        for c2 in connections.get(c).unwrap() {
            for c3 in connections.get(c).unwrap() {
                if connections.get(c).unwrap().contains(c2)
                    && connections.get(c2).unwrap().contains(&c3)
                    && connections.get(c3).unwrap().contains(c)
                {
                    let mut sorted = vec![c.as_str(), c2.as_str(), c3.as_str()];
                    sorted.sort();
                    let key = sorted.concat();

                    if !visited.contains(&key) {
                        if c.starts_with('t') || c2.starts_with('t') || c3.starts_with('t') {
                            solution1 += 1;
                        }
                        visited.insert(key.clone());
                        println!("{} {} {}", c, c2, c3);
                    }
                }
            }
        }
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut max:Vec<String>=Vec::new();
    for c in &comps {
        let group = get_connected(c, &connections, &mut visited);
        if group.len() > max.len() {
            max = group;
        }
    }
    max.sort();
    max.dedup();
    let password = max.concat();

    (solution1.to_string(), password)
}

fn get_connected(
    start: &String,
    connections: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> Vec<String> {
    if visited.contains(start) {
        return vec![start.clone()];
    }
    visited.insert(start.clone());
    let mut sum:Vec<String> = Vec::new();
    for c in connections.get(start).unwrap() {
        let mut nodes = get_connected(c, connections, visited);
        sum.append(&mut nodes);
    }
    sum
}
