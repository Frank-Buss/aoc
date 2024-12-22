use std::collections::HashMap;

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let mut solution2: u64 = 0;

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

    for d1 in -9..=9 {
        for d2 in -9..=9 {
            for d3 in -9..=9 {
                for d4 in -9..=9 {
                    let mut sum = 0 as u64;
                    let ds = vec![d1, d2, d3, d4];
                    for v_by_ds in &sellers {
                        if v_by_ds.contains_key(&ds) {
                            sum += *v_by_ds.get(&ds).unwrap() as u64;
                        }
                    }
                    if sum > solution2 {
                        solution2 = sum;
                    }
                }
            }
        }
    }

    (solution1.to_string(), solution2.to_string())
}
