use std::env;
use std::fs;
mod year2024;

pub fn not_found(_: Vec<String>) -> (i32, i32) {
    panic!("day not found");
}

fn read_lines(year: i32, day: i32) -> Vec<String> {
    let file = fs::read_to_string(format!("src/year{}/day{}.txt", year, day)).unwrap();
    file.lines().map(Into::into).collect()
}

fn call_day(year: i32, day: i32) -> (i32, i32) {
    // read input
    let lines = read_lines(year, day);

    // calculate solution
    let fun = match day {
        1 => year2024::day1::solve,
        2 => year2024::day2::solve,
        3 => year2024::day3::solve,
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

    #[test]
    fn test2024() {
        assert_eq!(call_day(2024, 1), (1834060, 21607792));
        assert_eq!(call_day(2024, 2), (510, 553));
        assert_eq!(call_day(2024, 3), (163931492, 76911921));
    }
}
