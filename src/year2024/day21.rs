use regex::Regex;

struct Context<'a> {
    k: &'a Vec<Vec<char>>,
    seqs: Vec<String>,
    shortest: usize,
    use_shortest: bool,
}

impl<'a> Context<'a> {
    fn new(k: &'a Vec<Vec<char>>, use_shortest: bool) -> Self {
        Self {
            k,
            seqs: Vec::new(),
            shortest: usize::MAX,
            use_shortest,
        }
    }

    fn find_seqs(&mut self, code: String, xs: i32, ys: i32, seq: String) {
        if code.len() > 0 {
            let c = code.as_bytes()[0] as char;
            'y: for y in 0..self.k.len() {
                for x in 0..self.k[0].len() {
                    if self.k[y][x] == c {
                        let x = x as i32;
                        let y = y as i32;
                        self.find_seqs2(code.clone(), xs, ys, x, y, seq.clone());
                        break 'y;
                    }
                }
            }
        }
    }

    fn find_seqs2(&mut self, code: String, xs: i32, ys: i32, x: i32, y: i32, seq: String) {
        let dx = x - xs;
        let dy = y - ys;
        if dx == 0 && dy == 0 {
            let code = code[1..].to_string();
            let seq = seq + "A";
            if self.use_shortest {
                if seq.len() >= self.shortest {
                    return;
                }
            }
            if code.len() > 0 {
                self.find_seqs(code, xs, ys, seq);
            } else {
                if !self.use_shortest {
                    self.seqs.push(seq);
                } else if seq.len() < self.shortest {
                    self.shortest = seq.len();
                    self.seqs.push(seq);
                }
            }
        } else {
            let x = x as i32;
            let y = y as i32;
            if dx < 0 {
                self.find_seqs2(code.clone(), xs - 1, ys, x, y, seq.clone() + "<");
            }
            if dx > 0 {
                self.find_seqs2(code.clone(), xs + 1, ys, x, y, seq.clone() + ">");
            }
            if dy < 0 {
                self.find_seqs2(code.clone(), xs, ys - 1, x, y, seq.clone() + "^");
            }
            if dy > 0 {
                self.find_seqs2(code.clone(), xs, ys + 1, x, y, seq.clone() + "v");
            }
        }
    }
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: usize = 0;

    #[rustfmt::skip]
    let nk = Vec::from([
        "789",
        "456",
        "123",
        " 0A"
    ].map(|s| s.chars().collect::<Vec<char>>()));

    #[rustfmt::skip]
    let dk = Vec::from([
        " ^A",
        "<v>"
    ].map(|s| s.chars().collect::<Vec<char>>()));

    let r = Regex::new(r"0*(\d+)").unwrap();
    for line in lines {
        if line.len() > 0 {
            let code = r.captures(&line).unwrap()[1].parse::<usize>().unwrap();

            // find all possible sequences for the last numeric keypad
            let mut context0 = Context::new(&nk, false);
            let xs = 2;
            let ys = 3;
            context0.find_seqs(line.clone(), xs, ys, "".to_string());
            println!("num seqs: {:?}", context0.seqs);

            // find all possible sequences for the first directional keypad
            let mut context1 = Context::new(&dk, false);
            let xs = 2;
            let ys = 0;
            for seq in &context0.seqs {
                context1.find_seqs(seq.clone(), xs, ys, "".to_string());
            }
            println!("dir seqs: {}", context1.seqs.len());

            // find all possible sequences for the second directional keypad, save only shortest sequence this time
            let mut context2 = Context::new(&dk, true);
            for seq in &context1.seqs {
                context2.find_seqs(seq.clone(), xs, ys, "".to_string());
            }

            // shortest sequence is any, because only the shortest are saved
            let p = &context2.seqs[0];
            println!("seq: {} code: {} {}", p.len(), code, p);

            solution1 += code * p.len();
        }
    }

    (solution1.to_string(), solution1.to_string())
}

#[cfg(test)]
mod tests {

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
        dir_keypad: &Vec<Vec<char>>,
        num_keypad: &Vec<Vec<char>>,
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
        let dir_keypad = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];

        let num_keypad = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec![' ', '0', 'A'],
        ];

        assert!(verify_full_sequence(
            "<<vAA>A>^AAvA<^A>AAvA^A<vA>^A<A>A<vA>^A<A>A<<vA>A>^AAvA<^A>A",
            "456A",
            &dir_keypad,
            &num_keypad
        ));
    }
}
