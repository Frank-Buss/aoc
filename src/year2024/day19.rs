use regex::Regex;
use trie_rs::{Trie, TrieBuilder};

use memoize::memoize;

static mut TOWELS: Option<Trie<u8>> = None;

#[memoize]
fn search(pattern: String, start: usize) -> bool {
    if start >= pattern.len() {
        return true;
    }
    let mut found = false;
    let results: Vec<String> = unsafe {
        TOWELS
            .as_ref()
            .unwrap()
            .common_prefix_search(pattern[start..].to_string())
            .collect()
    };
    for s in results {
        found |= search(pattern.clone(), start + s.len());
    }
    found
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;

    let mut line_iter = lines.iter();
    let line = line_iter.next().unwrap();
    let r = r"(\w+)";
    let r = Regex::new(&r).unwrap();
    let mut towels = TrieBuilder::new();
    for cap in r.find_iter(&line) {
        towels.push(cap.as_str().to_string());
    }
    let towels = towels.build();
    unsafe {
        TOWELS = Some(towels);
    }
    line_iter.next();

    loop {
        if let Some(pattern) = line_iter.next() {
            if search(pattern.clone(), 0) {
                solution1 += 1;
            }
        } else {
            break;
        }
    }

    (solution1.to_string(), "0".to_string())
}
