use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;

    let mut sellers: Vec<HashMap<Vec<i8>, u8>> = Vec::new();
    for line in lines {
        let mut vs: Vec<u8> = Vec::new();
        let mut ds: Vec<Vec<i8>> = Vec::new();
        if line.len() > 0 {
            let mut v = line.parse::<u64>().unwrap();
            let mut last = (v % 10) as u8;
            let mut ds2: Vec<i8> = Vec::new();
            for _ in 0..2000 {
                v = (v * 64 ^ v) % 16777216;
                v = (v / 32 ^ v) % 16777216;
                v = (v * 2048 ^ v) % 16777216;

                let p = (v % 10) as u8;
                vs.push(p);
                let d = (p as i8) - (last as i8);
                ds2.push(d);
                if ds2.len() > 4 {
                    ds2.remove(0);
                }
                last = p;
                ds.push(ds2.clone());
            }
            solution1 += v;
        }
        let mut v_by_ds: HashMap<Vec<i8>, u8> = HashMap::new();
        for i in 0..vs.len() {
            let ds = &ds[i];
            if !v_by_ds.contains_key(ds) {
                v_by_ds.insert(ds.clone(), vs[i]);
            }
        }
        sellers.push(v_by_ds);
    }

    let sellers = Arc::new(sellers);

    let solution2 = (-9..=9)
        .into_par_iter()
        .flat_map(|d1| {
            let sellers = Arc::clone(&sellers);
            (-9..=9).into_par_iter().flat_map(move |d2| {
                let sellers = Arc::clone(&sellers);
                (-9..=9).into_par_iter().flat_map(move |d3| {
                    let sellers = Arc::clone(&sellers);
                    (-9..=9).into_par_iter().map(move |d4| {
                        let ds = vec![d1, d2, d3, d4];
                        sellers
                            .iter()
                            .filter_map(|v_by_ds| v_by_ds.get(&ds))
                            .map(|&v| v as u64)
                            .sum::<u64>()
                    })
                })
            })
        })
        .max()
        .unwrap_or(0);

    (solution1.to_string(), solution2.to_string())
}
