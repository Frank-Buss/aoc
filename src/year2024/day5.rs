use std::collections::HashMap;

pub fn search(
    result: &Vec<i32>,
    rules: &HashMap<i32, Vec<i32>>,
    update: &Vec<i32>,
    prev_after: i32,
    final_result: &mut Vec<i32>
) {
    for (key, value) in rules.into_iter() {
        for before in update {
            if *before == *key
                && ((prev_after == *before) || (prev_after == 0))
            {
                let mut update = update.clone();
                update.retain(|&x| x != *before);
                for after in &update {
                    if value.contains(after) {
                        let mut result = result.clone();
                        result.push(*before);
                        if update.len() == 1 {
                            result.push(*after);
                            *final_result = result.clone();
                            return;
                        } else {
                            search(&mut result, rules, &update, *after, final_result);
                        }
                    }
                }
            }
        }
    }
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut solution1 = 0;
    let mut solution2 = 0;

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let parts: Vec<String> = line.split("|").map(str::to_string).collect();
        if parts.len() > 1 {
            let parts: Vec<i32> = parts
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if parts.len() == 2 {
                rules.entry(parts[0]).or_default().push(parts[1]);
            }
        }

        let parts: Vec<String> = line.split(",").map(str::to_string).collect();
        if parts.len() > 1 {
            let parts: Vec<i32> = parts
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if parts.len() > 0 {
                updates.push(parts);
            }
        }
    }

    for update in updates {
        let mut ok = true;
        for i in 0..update.len() - 1 {
            let before = update[i];
            for j in i + 1..update.len() {
                let after = update[j];
                let found = rules.entry(before).or_default().contains(&after);
                if !found {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            solution1 += update[update.len() / 2];
        } else {
            println!("test {:?}", update);
            let mut result = Vec::new();
            let mut final_result = Vec::new();
            search(&mut result, &rules, &update, 0, &mut final_result);
            if final_result.len() > 0 {
                println!("sorted {:?}", final_result);
                solution2 += final_result[final_result.len() / 2];
            }
        }
    }

    // return solutions
    (solution1, solution2)
}
