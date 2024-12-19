use memoize::memoize;
use regex::Regex;
use std::sync::LazyLock;
use std::sync::Mutex;
use trie_rs::{Trie, TrieBuilder};

static TOWELS: LazyLock<Mutex<Trie<u8>>> = LazyLock::new(|| Mutex::new(TrieBuilder::new().build()));

#[memoize]
fn search(pattern: String, start: usize) -> usize {
    if start >= pattern.len() {
        return 1;
    }
    let results: Vec<String> = {
        let towels = TOWELS.lock().unwrap();
        towels
            .common_prefix_search(pattern[start..].to_string())
            .collect()
    };
    let mut found: usize = 0;
    for s in results {
        found += search(pattern.clone(), start + s.len());
    }
    found
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: usize = 0;
    let mut solution2: usize = 0;

    let mut line_iter = lines.iter();
    let line = line_iter.next().unwrap();
    let r = r"(\w+)";
    let r = Regex::new(&r).unwrap();
    let mut towels = TrieBuilder::new();
    for cap in r.find_iter(&line) {
        towels.push(cap.as_str().to_string());
    }
    let built_towels = towels.build();
    *TOWELS.lock().unwrap() = built_towels;
    line_iter.next();

    loop {
        if let Some(pattern) = line_iter.next() {
            let combinations = search(pattern.clone(), 0);
            if combinations > 0 {
                solution1 += 1;
            }
            solution2 += combinations;
        } else {
            break;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
