use num::bigint::BigInt;

fn test1(i: usize, sum: &BigInt, acc: &BigInt, numbers: &Vec<BigInt>) -> bool {
    let acc = if i == 0 { &numbers[0] } else { acc };
    let i = i + 1;
    if i == numbers.len() {
        return acc == sum;
    }
    let next = &numbers[i];
    test1(i, sum, &(acc * next), numbers) || test1(i, sum, &(acc + next), numbers)
}

fn test2(i: usize, sum: &BigInt, acc: &BigInt, numbers: &Vec<BigInt>) -> bool {
    let acc = if i == 0 { &numbers[0] } else { acc };
    let i = i + 1;
    if i == numbers.len() {
        return acc == sum;
    }
    let next = &numbers[i];
    test2(i, sum, &(acc * next), numbers)
        || test2(i, sum, &(acc + next), numbers)
        || test2(
            i,
            sum,
            &(acc.to_string() + &next.to_string())
                .parse::<BigInt>()
                .unwrap(),
            numbers,
        )
}

pub fn solve(lines: Vec<String>) -> (String, String) {
    let mut solution1: BigInt = 0.into();
    let mut solution2: BigInt = 0.into();

    for line in lines {
        let mut parts = line.split(':');
        let sum = parts.next().unwrap().parse::<BigInt>().unwrap();
        let numbers = parts.next().unwrap().trim().split_whitespace();
        let numbers: Vec<BigInt> = numbers.map(|x| x.parse::<BigInt>().unwrap()).collect();
        if test1(0, &sum, &0.into(), &numbers) {
            solution1 += &sum;
        }
        if test2(0, &sum, &0.into(), &numbers) {
            solution2 += &sum;
        }
    }

    (solution1.to_string(), solution2.to_string())
}
