use itertools::Itertools;
use regex::Regex;

fn read_ints(line: &String) -> Vec<u64> {
    let r = r"(\d+)";
    let mut ints: Vec<u64> = Vec::new();
    let r = Regex::new(&r).unwrap();
    for cap in r.find_iter(&line) {
        ints.push(cap.as_str().parse::<u64>().unwrap());
    }
    ints
}

fn run(prg: &Vec<u8>, out: &mut Vec<u8>, rega: u64, regb: u64, regc: u64) -> usize {
    let mut rega = rega;
    let mut regb = regb;
    let mut regc = regc;
    let mut i = 0;
    let mut outi = 0;
    loop {
        let opcode = prg[i] as u64;
        i += 1;
        let operand = prg[i] as u64;
        i += 1;
        let combo = match operand {
            0..=3 => operand,
            4 => rega,
            5 => regb,
            6 => regc,
            _ => 0,
        };
        match opcode {
            0 => {
                // adv
                rega = rega / (1 << combo);
            }
            1 => {
                // bxl
                regb = regb ^ operand;
            }
            2 => {
                // bst
                regb = combo & 7;
            }
            3 => {
                // jnz
                if rega != 0 {
                    i = operand as usize;
                }
            }
            4 => {
                // bxc
                regb ^= regc;
            }
            5 => {
                // out
                out[outi] = (combo & 7) as u8;
                outi += 1;
                if outi >= prg.len() {
                    return outi;
                }
            }
            6 => {
                // bdv
                regb = rega / (1 << combo);
            }
            7 => {
                // cdv
                regc = rega / (1 << combo);
            }
            _ => {}
        }
        if i >= prg.len() {
            break;
        }
    }
    outi
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut i = lines.iter();
    let rega = read_ints(i.next().unwrap())[0];
    let regb = read_ints(i.next().unwrap())[0];
    let regc = read_ints(i.next().unwrap())[0];
    i.next();
    let prg64 = read_ints(i.next().unwrap());
    let mut prg = Vec::new();
    for i in prg64 {
        prg.push(i as u8);
    }

    let mut out: Vec<u8> = vec![0; 100];
    let len = run(&prg, &mut out, rega, regb, regc);
    let solution1 = Itertools::join(&mut out[0..len].iter(), ",");

    let mut solution2 = "".to_string();
    let mut out: Vec<u8> = vec![0; prg.len()];
    for a in 0.. {
        let len = run(&prg, &mut out, a, regb, regc);
        if len == prg.len() && out == prg {
            solution2 = a.to_string();
            break;
        }
        if a % 1000000 == 0 {
            println!("{}", a);
        }
    }

    (solution1, solution2)
}
