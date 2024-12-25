use pathfinding::prelude::dijkstra;
use regex::Regex;

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
    fn mov(&mut self, dir: char) {
        match dir {
            '<' => {
                self.x -= 1;
            }
            '>' => {
                self.x += 1;
            }
            '^' => self.y -= 1,
            'v' => self.y += 1,
            _ => {}
        }
    }

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
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    dir1: Point,
    dir2: Point,
    num: Point,
    code: String,
    dir: char,
}

impl Pos {
    fn try_key(&self, result: &mut Vec<Pos>, c: char) {
        let mut next = self.clone();
        let pressed = self.dir == 'A';
        next.dir = c;
        let c1 = DIR_PAD[self.dir1.y as usize][self.dir1.x as usize];

        if c == 'A' {
            if !pressed || true {
                // button pressed on first dir pad
                if c1 == 'A' {
                    // button pressed on second dir pad
                    let c2 = DIR_PAD[self.dir2.y as usize][self.dir2.x as usize];

                    if c2 == 'A' {
                        // button pressed on num pad, add to code
                        // max code length is 4 and ends in A
                        let n = NUM_PAD[self.num.y as usize][self.num.x as usize];
                        next.code.push(n);
                        if next.code.len() <= 4 {
                            if next.code.len() == 4 {
                                if n == 'A' {
                                    result.push(next);
                                }
                            } else {
                                if n != 'A' {
                                    result.push(next);
                                }
                            }
                        }
                    } else {
                        // move on num pad
                        next.num.mov(c2);
                        if next.num.is_valid(&NUM_PAD) {
                            result.push(next);
                        }
                    }
                } else {
                    // move on second dir pad
                    next.dir2.mov(c1);
                    if next.dir2.is_valid(&DIR_PAD) {
                        result.push(next);
                    }
                }
            }
        } else {
            // move on first dir pad
            next.dir1.mov(c);
            if next.dir1.is_valid(&DIR_PAD) {
                result.push(next);
            }
        }
    }

    fn successors(&self) -> Vec<(Pos, usize)> {
        let mut result = Vec::new();
        self.try_key(&mut result, '<');
        self.try_key(&mut result, '>');
        self.try_key(&mut result, 'v');
        self.try_key(&mut result, '^');
        self.try_key(&mut result, 'A');
        result.into_iter().map(|p| (p, 1)).collect()
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: usize = 0;

    let r = Regex::new(r"0*(\d+)").unwrap();
    for code in lines {
        if code.len() > 0 {
            let start = Pos {
                dir1: Point { x: 2, y: 0 },
                dir2: Point { x: 2, y: 0 },
                num: Point { x: 2, y: 3 },
                code: "".into(),
                dir: ' ',
            };
            let path = dijkstra(&start, |p| p.successors(), |p| *p.code == code)
                .unwrap()
                .0;
            let dirs = path.iter().map(|p| p.dir).collect::<String>();

            let code_number = r.captures(&code).unwrap()[1].parse::<usize>().unwrap();
            solution1 += code_number * (dirs.len() - 1);
        }
    }

    (solution1.to_string(), solution1.to_string())
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
