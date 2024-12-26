use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::hash::{Hash, Hasher};

const DIR_PAD: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

const NUM_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_valid(&self, pad: &[[char; 3]]) -> bool {
        let w: i32 = pad[0].len() as i32;
        let h: i32 = pad.len() as i32;
        let x = self.x;
        let y = self.y;
        if x >= 0 && y >= 0 && x < w && y < h {
            pad[y as usize][x as usize] != ' '
        } else {
            false
        }
    }

    fn mov(&mut self, dir: char) {
        match dir {
            '<' => self.x -= 1,
            '>' => self.x += 1,
            '^' => self.y -= 1,
            'v' => self.y += 1,
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Ord, PartialOrd)]
struct Pos {
    dirs: Vec<Point>,
    num: Point,
    code: String,
    dir: char,
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dirs.hash(state);
        self.num.hash(state);
        self.code.hash(state);
        // exclude self.dir
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.dirs == other.dirs && self.num == other.num && self.code == other.code
        // exclude self.dir
    }
}

impl Eq for Pos {}

impl Pos {
    fn successors(&self, code: &String) -> Vec<(Pos, usize)> {
        let mut result = Vec::new();
        for c in ['<', '>', 'v', '^', 'A'] {
            let mut next = self.clone();
            next.press_key(c);
            if next.is_valid(code) {
                result.push(next);
            }
        }
        result.into_iter().map(|p| (p, 1)).collect()
    }

    fn press_key(&mut self, c: char) {
        // set top level key
        self.dir = c;

        // iterate all key pads
        let mut c = c;
        for i in 0..self.dirs.len() {
            let is_last = i == self.dirs.len() - 1;
            let dir = &mut self.dirs[i];
            if c == 'A' {
                // A pressed
                c = DIR_PAD[dir.y as usize][dir.x as usize];
                if is_last {
                    break;
                } else {
                    // press next key pad
                    c = DIR_PAD[dir.y as usize][dir.x as usize];
                }
            } else {
                // direction key pressed, move on current pad, no action for next pads needed
                dir.mov(c);
                return;
            }
        }

        // press key or move on num pad
        if c == 'A' {
            // button pressed on num pad, add to code
            let n = NUM_PAD[self.num.y as usize][self.num.x as usize];
            self.code.push(n);
        } else {
            // movement on num pad
            self.num.mov(c);
        }
    }

    fn is_valid(&self, code: &String) -> bool {
        self.is_valid_code(code)
            && self.dirs.iter().all(|d| d.is_valid(&DIR_PAD))
            && self.num.is_valid(&NUM_PAD)
    }

    fn is_valid_code(&self, code: &String) -> bool {
        // must be 4 chars
        if self.code.len() > 4 {
            return false;
        }

        // first 3 chars must be digits
        if !self.code.chars().take(3).all(|c| c.is_digit(10)) {
            return false;
        }

        // last char must be A
        if self.code.len() == 4 {
            return self.code.chars().last().unwrap() == 'A';
        }

        // must start with the target code
        code.starts_with(&self.code);

        true
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    for i in 1..25 {
        println!("{i} {}", solve_codes(&lines, i).to_string());
    }

    (
        solve_codes(&lines, 2).to_string(),
        "".to_string(), //solve_codes(&lines, 25).to_string(),
    )
}

fn solve_codes(lines: &Vec<String>, num_pad_count: usize) -> usize {
    let mut solution: usize = 0;
    let r = Regex::new(r"0*(\d+)").unwrap();
    for code in lines {
        if code.len() > 0 {
            let start = Pos {
                dirs: vec![Point { x: 2, y: 0 }; num_pad_count],
                num: Point { x: 2, y: 3 },
                code: "".into(),
                dir: ' ',
            };
            let path = dijkstra(&start, |p| p.successors(&code), |p| *p.code == *code)
                .unwrap()
                .0;
            let dirs = path.iter().map(|p| p.dir).collect::<String>();

            let code_number = r.captures(&code).unwrap()[1].parse::<usize>().unwrap();
            solution += code_number * (dirs.len() - 1);
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    fn move_on_pad(x: &mut i32, y: &mut i32, c: char) {
        match c {
            '<' => *x -= 1,
            '>' => *x += 1,
            '^' => *y -= 1,
            'v' => *y += 1,
            _ => {}
        }
    }

    fn verify_full_sequence(
        sequence: &str,
        code: &str,
        dir_keypad: &[[char; 3]; 2],
        num_keypad: &[[char; 3]; 4],
    ) -> bool {
        // Start positions for all pads
        let mut dir1_x: i32 = 2; // First directional pad, start at 'A'
        let mut dir1_y: i32 = 0;

        let mut dir2_x: i32 = 2; // Second directional pad
        let mut dir2_y: i32 = 0;

        let mut num_x: i32 = 2; // Number pad
        let mut num_y: i32 = 3;

        // Process first directional pad moves
        let mut result: Vec<char> = Vec::new();
        for c in sequence.chars() {
            move_on_pad(&mut dir1_x, &mut dir1_y, c);
            if c == 'A' {
                // Get the character at current position on dir1
                let next_move = dir_keypad[dir1_y as usize][dir1_x as usize];

                // Process second directional pad based on first pad's position
                move_on_pad(&mut dir2_x, &mut dir2_y, next_move);
                if next_move == 'A' {
                    // Get the character at current position on dir2
                    let final_move = dir_keypad[dir2_y as usize][dir2_x as usize];

                    // Process number pad based on second pad's position
                    move_on_pad(&mut num_x, &mut num_y, final_move);
                    if final_move == 'A' {
                        // Get the final number
                        result.push(num_keypad[num_y as usize][num_x as usize]);
                    }
                }
            }
        }
        let result = result.iter().collect::<String>();
        result == code
    }

    #[test]
    fn test_verify_full_sequence() {
        assert!(verify_full_sequence(
            "<<vAA>A>^AAvA<^A>AAvA^A<vA>^A<A>A<vA>^A<A>A<<vA>A>^AAvA<^A>A",
            "456A",
            &DIR_PAD,
            &NUM_PAD
        ));
    }
}
