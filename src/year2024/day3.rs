pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut state = 0;

    let mut solution1 = 0;
    let mut solution2 = 0;
    let mut enabled = true;
    for line in &lines {
        let mut num1: String = "".into();
        let mut num2: String = "".into();
        for c in line.chars() {
            match state {
                0 => {
                    if c == 'm' {
                        state += 1;
                    } else if c == 'd' {
                        state = 6;
                    }
                }
                1 => {
                    if c == 'u' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                2 => {
                    if c == 'l' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                3 => {
                    if c == '(' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                4 => {
                    if c.is_digit(10) {
                        num1.push(c);
                    } else if c == ',' {
                        state += 1;
                    } else {
                        num1.clear();
                        state = 0;
                    }
                }
                5 => {
                    if c.is_digit(10) {
                        num2.push(c);
                    } else if c == ')' {
                        let add = num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
                        solution1 += add;
                        if enabled {
                            solution2 += add;
                        }
                        num1.clear();
                        num2.clear();
                        state = 0;
                    } else {
                        num1.clear();
                        num2.clear();
                        state = 0;
                    }
                }
                6 => {
                    if c == 'o' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                7 => {
                    if c == 'n' {
                        state = 9;
                    } else if c == '(' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                8 => {
                    if c == ')' {
                        enabled = true;
                    }
                    state = 0;
                }
                9 => {
                    if c == '\'' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                10 => {
                    if c == 't' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                11 => {
                    if c == '(' {
                        state += 1;
                    } else {
                        state = 0;
                    }
                }
                12 => {
                    if c == ')' {
                        enabled = false;
                    }
                    state = 0;
                }
                _ => {}
            }
        }
    }

    // return solutions
    (solution1, solution2)
}
