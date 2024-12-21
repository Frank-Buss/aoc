use regex::Regex;

struct Context<'a> {
    k: &'a Vec<Vec<char>>,
    paths: Vec<String>,
    shortest: usize,
    use_shortest: bool,
}

impl<'a> Context<'a> {
    fn new(k: &'a Vec<Vec<char>>, use_shortest: bool) -> Self {
        Self {
            k,
            paths: Vec::new(),
            shortest: usize::MAX,
            use_shortest,
        }
    }

    fn find_paths(&mut self, code: String, xs: i32, ys: i32, path: String) {
        if code.len() > 0 {
            let c = code.as_bytes()[0] as char;
            'y: for y in 0..self.k.len() {
                for x in 0..self.k[0].len() {
                    if self.k[y][x] == c {
                        let x = x as i32;
                        let y = y as i32;
                        self.find_paths2(code.clone(), xs, ys, x, y, path.clone());
                        break 'y;
                    }
                }
            }
        }
    }

    fn find_paths2(&mut self, code: String, xs: i32, ys: i32, x: i32, y: i32, path: String) {
        let dx = x - xs;
        let dy = y - ys;
        if dx == 0 && dy == 0 {
            let code = code[1..].to_string();
            let path = path + "A";
            if self.use_shortest {
                if path.len() >= self.shortest {
                    return;
                }
            }
            if code.len() > 0 {
                self.find_paths(code, xs, ys, path);
            } else {
                if !self.use_shortest {
                    self.paths.push(path);
                } else if path.len() < self.shortest {
                    self.shortest = path.len();
                    self.paths.push(path);
                }
            }
        } else {
            let x = x as i32;
            let y = y as i32;
            if dx < 0 {
                self.find_paths2(code.clone(), xs - 1, ys, x, y, path.clone() + "<");
            }
            if dx > 0 {
                self.find_paths2(code.clone(), xs + 1, ys, x, y, path.clone() + ">");
            }
            if dy < 0 {
                self.find_paths2(code.clone(), xs, ys - 1, x, y, path.clone() + "^");
            }
            if dy > 0 {
                self.find_paths2(code.clone(), xs, ys + 1, x, y, path.clone() + "v");
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

            // find all possible paths for the last numeric keypad
            let mut context0 = Context::new(&nk, false);
            context0.find_paths(line.clone(), 3, 2, "".to_string());
            println!("num paths: {:?}", context0.paths);

            // find all possible paths for the first directional keypad
            let mut context1 = Context::new(&dk, false);
            for path in &context0.paths {
                context1.find_paths(path.clone(), 2, 0, "".to_string());
            }
            println!("dir paths: {}", context1.paths.len());

            // find all possible paths for the second directional keypad, save only shortest paths this time
            let mut context2 = Context::new(&dk, true);
            for (i, path) in context1.paths.iter().enumerate() {
                println!("{}: {}", i, path);
                context2.find_paths(path.clone(), 2, 0, "".to_string());
            }

            // find shortest path and use its length
            let p = &context2.paths[0];
            println!("{}", p);

            solution1 += code * p.len();
        }
    }

    (solution1.to_string(), solution1.to_string())
}
