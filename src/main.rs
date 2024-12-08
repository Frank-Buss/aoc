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
    }
}
