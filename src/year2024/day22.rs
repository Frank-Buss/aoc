#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Seller {
    vs: Vec<u8>,
    ds: Vec<Vec<i8>>,
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: u64 = 0;
    let mut solution2: u64 = 0;

    let mut sellers: Vec<Seller> = Vec::new();
    for line in lines {
        let mut vs: Vec<u8> = Vec::new();
        let mut ds: Vec<Vec<i8>> = Vec::new();
        let mut last: u8 = 0;
        if line.len() > 0 {
            let mut v = line.parse::<u64>().unwrap();
            last = (v % 10) as u8;
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
        sellers.push(Seller { vs, ds });
    }

    let mut max = 0;
    for d1 in -9..=9 {
        for d2 in -9..=9 {
            for d3 in -9..=9 {
                for d4 in -9..=9 {
                    if d1 != 0 && d2 != 0 && d3 != 0 && d4 != 0 {
                        let mut sum = 0;
                        let ds = vec![d1, d2, d3, d4];
                        for s in &sellers {
                            let mut v = 0;
                            for i in 0..s.vs.len() {
                                if s.ds[i] == ds {
                                    v = s.vs[i];
                                    break;
                                }
                            }
                            sum += v;
                        }
                        if sum > max {
                            max = sum;
                        }
                    }
                }
            }
        }
    }

    (solution1.to_string(), max.to_string())
}
