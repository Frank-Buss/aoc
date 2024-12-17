use itertools::Itertools;
use regex::Regex;

fn read_ints(line: &String) -> Vec<i64> {
    let r = r"(\d+)";
    let mut ints: Vec<i64> = Vec::new();
    let r = Regex::new(&r).unwrap();
    for cap in r.find_iter(&line) {
        ints.push(cap.as_str().parse::<i64>().unwrap());
    }
    ints
}

fn run(prg: &Vec<i64>, rega: i64, regb: i64, regc: i64) -> Vec<i64> {
    let mut rega = rega;
    let mut regb = regb;
    let mut regc = regc;
    let mut i = 0;
    let mut out: Vec<i64> = Vec::new();
    loop {
        let opcode = prg[i];
        i += 1;
        let operand = prg[i];
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
                out.push(combo & 7);
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
    out
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut i = lines.iter();
    let rega = read_ints(i.next().unwrap())[0];
    let regb = read_ints(i.next().unwrap())[0];
    let regc = read_ints(i.next().unwrap())[0];
    i.next();
    let prg = read_ints(i.next().unwrap());

    let out = run(&prg, rega, regb, regc);
    let solution1 = Itertools::join(&mut out.iter(), ",");

    let mut solution2 = "".to_string();
    for a in 0.. {
        let out = run(&prg, a, regb, regc);
        if out == prg {
            solution2 = a.to_string();
        }
        if a % 1000000 == 0 {
            println!("{}",a);
        }
    }

    (solution1, solution2)
}
