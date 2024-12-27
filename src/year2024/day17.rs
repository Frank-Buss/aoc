use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::collections::HashMap;

fn read_ints(line: &String) -> Vec<u64> {
    let r = r"(\d+)";
    let mut ints: Vec<u64> = Vec::new();
    let r = Regex::new(&r).unwrap();
    for cap in r.find_iter(&line) {
        ints.push(cap.as_str().parse::<u64>().unwrap());
    }
    ints
}

fn run(prg: &Vec<u8>, rega: u64, regb: u64, regc: u64) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut rega = rega;
    let mut regb = regb;
    let mut regc = regc;
    let mut pc = 0;
    loop {
        let opcode = prg[pc] as u64;
        pc += 1;
        let operand = prg[pc] as u64;
        pc += 1;
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
                rega = rega >> combo;
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
                    pc = operand as usize;
                }
            }
            4 => {
                // bxc
                regb ^= regc;
            }
            5 => {
                // out
                out.push((combo & 7) as u8);
                if out.len() > prg.len() {
                    break;
                }
            }
            6 => {
                // bdv
                regb = rega >> combo;
            }
            7 => {
                // cdv
                regc = rega >> combo;
            }
            _ => {}
        }
        if pc >= prg.len() {
            break;
        }
    }
    out
}

fn print_regs(rega: u64, regb: u64, regc: u64) {
    println!("a: 0x{rega:08x}  b: 0x{regb:08x}  c: 0x{regc:08x}");
}

fn print_line(pc: usize, opcode: u8, operand: u8) {
    print!("{pc:02}: ");
    print!("{opcode} {operand}    ");
    let combo = match operand {
        0..=3 => operand.to_string(),
        4 => "a".to_string(),
        5 => "b".to_string(),
        6 => "c".to_string(),
        _ => "".to_string(),
    };
    match opcode {
        0 => println!("adv {combo}  ; a = a >> {combo}"),
        1 => println!("bxl {operand}  ; b = b ^ {operand}"),
        2 => println!("bst {combo}  ; b = {combo} & 7"),
        3 => println!("jnz {operand}"),
        4 => println!("bxc    ; b ^= c"),
        5 => println!("out {combo}"),
        6 => println!("bdv {combo}  ; b = a >> {combo}"),
        7 => println!("cdv {combo}  ; c = a >> {combo}"),
        _ => {}
    }
}

fn disassemble(prg: &Vec<u8>, rega: u64, regb: u64, regc: u64) {
    print_regs(rega, regb, regc);
    let mut pc = 0;
    loop {
        let opcode = prg[pc];
        pc += 1;
        let operand = prg[pc];
        pc += 1;
        print_line(pc - 2, opcode, operand);
        if pc >= prg.len() {
            break;
        }
    }
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

    let out = run(&prg, rega, regb, regc);
    let solution1 = Itertools::join(&mut out.iter(), ",");

    disassemble(&prg, rega, regb, regc);
    println!();

    // first 10 bits of "A" determines the output, create all possible values for a given output
    let mut vs: HashMap<u8, Vec<u16>> = HashMap::new();
    for o in 0..8 {
        for a in 0..1024 {
            let out = run(&prg, a, regb, regc);
            if o == out[0] {
                vs.entry(o).or_default().push(a as u16);
            }
        }
    }

    // use Dijkstra's algorithm with 'A' value as cost
    let solution2 = dijkstra(
        &(0u64, 0usize),
        |&(a, pc): &(u64, usize)| {
            let mut next = Vec::new();
            let shift = pc * 3;
            let upper = a >> shift;

            // search all possible values which can produce the next 3 bits output
            for &x in vs.get(&prg[pc]).unwrap() {
                let x = x as u64;
                if pc > 0 {
                    if pc <= prg.len() {
                        // the previous 7 upper bits must overlap with the 7 lower bits of the new value
                        // because there are 10 relevant bits for a value, and 'A' is shifted right by 3
                        // bits for each iteration
                        if x & 127 == upper {
                            let a = a | (x << shift);
                            next.push(((a, pc + 1), a));
                        }
                    }
                } else {
                    let a = x;
                    next.push(((a, pc + 1), a));
                }
            }
            next
        },
        |&(a, _)| prg == run(&prg, a, regb, regc),
    )
    .unwrap();

    (solution1, solution2.0.last().unwrap().0.to_string())
}
