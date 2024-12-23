use itertools::Itertools;
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
                    }
                }
            }
        }
    }

    // get max number of connections for one node
    let max = connections
        .values()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();

    // assume this is also the largest interconnected group,
    // with one outgoing connection per node outside the group
    let mut password = "".to_string();
    for c in &comps {
        let cs = connections.get(c).unwrap();
        if cs.len() == max {
            // check all subsequence combinations for one less connection
            for cs2 in cs.iter().combinations(max - 1) {
                // check if this subgroup is fully interconnected
                let mut connected = true;
                for cg2 in cs2.iter().combinations(2) {
                    // each pair needs to be connected, and needs to have a connection to the root node
                    if !connections.get(*cg2[0]).unwrap().contains(cg2[1]) {
                        connected = false;
                        break;
                    }
                    if !connections.get(*cg2[0]).unwrap().contains(c) {
                        connected = false;
                        break;
                    }
                }
                if connected {
                    let mut result = cs2.clone();
                    result.push(&c);
                    result.sort();
                    password = result.into_iter().join(",");
                }
            }
        }
    }

    (solution1.to_string(), password)
}
