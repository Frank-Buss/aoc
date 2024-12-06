use std::collections::{HashMap, HashSet, VecDeque};

// Performs topological sort of numbers based on dependency rules
// nums: slice of numbers to sort
// rules: map of number -> set of numbers that must come after it
fn topo_sort(nums: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let set: HashSet<_> = nums.iter().copied().collect();

    // Calculate in-degree (number of dependencies) for each number
    let mut deps: HashMap<i32, usize> = nums
        .iter()
        .flat_map(|&n| {
            rules
                .get(&n)
                .into_iter()
                .flatten()
                .filter(|&next| set.contains(next))
        })
        .fold(HashMap::new(), |mut m: HashMap<i32, usize>, &n| {
            *m.entry(n).or_default() += 1;
            m
        });

    // Start with nodes that have no dependencies
    let mut q: VecDeque<_> = set
        .iter()
        .filter(|n| !deps.contains_key(n))
        .copied()
        .collect();

    let mut res = Vec::with_capacity(nums.len());

    // Process nodes in order, removing dependencies as we go
    while let Some(n) = q.pop_front() {
        res.push(n);
        if let Some(nexts) = rules.get(&n) {
            for &next in nexts.iter().filter(|&next| set.contains(next)) {
                if let Some(c) = deps.get_mut(&next) {
                    *c -= 1;
                    if *c == 0 {
                        q.push_back(next);
                    }
                }
            }
        }
    }
    res
}

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    // Parse input into rules (n1|n2) and updates (n1,n2,...)
    let (rules, updates): (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) = lines.iter().fold(
        (HashMap::new(), Vec::new()),
        |(mut rules, mut updates), line| {
            // Parse dependency rules (format: n1|n2 means n1 must come before n2)
            if let [n1, n2] = line
                .split('|')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>()[..]
            {
                rules.entry(n1).or_default().insert(n2);
            }

            // Parse update sequences (comma-separated numbers)
            let nums: Vec<_> = line.split(',').filter_map(|s| s.parse().ok()).collect();
            if !nums.is_empty() {
                updates.push(nums);
            }

            (rules, updates)
        },
    );

    // Process each update sequence:
    // - If valid (follows rules), sum middle number into s1
    // - If invalid, sort topologically and sum middle number into s2
    updates.into_iter().fold((0, 0), |(s1, s2), update| {
        let valid = (0..update.len() - 1).all(|i| {
            (i + 1..update.len()).all(|j| {
                rules
                    .get(&update[i])
                    .map_or(false, |set| set.contains(&update[j]))
            })
        });

        let mid = update[update.len() / 2];
        if valid {
            (s1 + mid, s2)
        } else {
            let ordered = topo_sort(&update, &rules);
            (s1, s2 + ordered[ordered.len() / 2])
        }
    })
}
