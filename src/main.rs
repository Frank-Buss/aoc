use std::env;
use std::fs;
mod year2024;

pub fn not_found(_: Vec<String>) -> (String, String) {
    panic!("day not found");
}

fn read_lines(year: i32, day: i32) -> Vec<String> {
    let file = fs::read_to_string(format!("src/year{}/day{}.txt", year, day)).unwrap();
    file.lines().map(Into::into).collect()
}

fn call_day(year: i32, day: i32) -> (String, String) {
    // read input
    let lines = read_lines(year, day);

    // calculate solution
    let fun = match day {
        1 => year2024::day1::solve,
        2 => year2024::day2::solve,
        3 => year2024::day3::solve,
        4 => year2024::day4::solve,
        5 => year2024::day5::solve,
        6 => year2024::day6::solve,
        7 => year2024::day7::solve,
        8 => year2024::day8::solve,
        9 => year2024::day9::solve,
        10 => year2024::day10::solve,
        11 => year2024::day11::solve,
        12 => year2024::day12::solve,
        13 => year2024::day13::solve,
        14 => year2024::day14::solve,
        15 => year2024::day15::solve,
        16 => year2024::day16::solve,
        17 => year2024::day17::solve,
        18 => year2024::day18::solve,
        19 => year2024::day19::solve,
        20 => year2024::day20::solve,
        21 => year2024::day21::solve,
        22 => year2024::day22::solve,
        23 => year2024::day23::solve,
        24 => year2024::day24::solve,
        25 => year2024::day25::solve,
        _ => not_found,
    };
    fun(lines)
}

fn main() {
    // parse command line arguments
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<i32>().unwrap();
    let day = args[2].parse::<i32>().unwrap();

    // call solve function of the specified day
    let (solution1, solution2) = call_day(year, day);

    // show solution
    println!("solution 1: {}", solution1);
    println!("solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {
        ($year:expr, $day:expr, $s1:expr, $s2:expr) => {
            assert_eq!(call_day($year, $day), ($s1.to_string(), $s2.to_string()))
        };
    }

    #[test]
    fn test2024() {
        test_case!(2024, 1, "1834060", "21607792");
        test_case!(2024, 2, "510", "553");
        test_case!(2024, 3, "163931492", "76911921");
        test_case!(2024, 4, "2633", "1936");
        test_case!(2024, 5, "5129", "4077");
        test_case!(2024, 6, "5153", "1711");
        test_case!(2024, 7, "1399219271639", "275791737999003");
        test_case!(2024, 8, "276", "991");
        test_case!(2024, 9, "6446899523367", "6478232739671");
        test_case!(2024, 10, "667", "1344");
        test_case!(2024, 11, "218956", "259593838049805");
        test_case!(2024, 12, "1461752", "904114");
        test_case!(2024, 13, "34393", "83551068361379");
        test_case!(2024, 14, "229980828", "7132");
        test_case!(2024, 15, "1441031", "1425169");
        test_case!(2024, 16, "0", "0");
        test_case!(2024, 17, "0", "0");
        test_case!(2024, 18, "0", "0");
        test_case!(2024, 19, "0", "0");
        test_case!(2024, 20, "0", "0");
        test_case!(2024, 21, "0", "0");
        test_case!(2024, 22, "0", "0");
        test_case!(2024, 23, "0", "0");
        test_case!(2024, 24, "0", "0");
        test_case!(2024, 25, "0", "0");
    }
}
