mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::env;

fn main() {
    let day = env::args().nth(1).unwrap();
    let filename = env::args().nth(2);
    match day.as_ref() {
        "1.1" => day01::star1(&filename.unwrap_or("inputs/day01.txt".to_string())),
        "1.2" => day01::star2(&filename.unwrap_or("inputs/day01.txt".to_string())),
        "2.1" => day02::star1(&filename.unwrap_or("inputs/day02.txt".to_string())),
        "2.2" => day02::star2(&filename.unwrap_or("inputs/day02.txt".to_string())),
        "3.1" => day03::star1(&filename.unwrap_or("inputs/day03.txt".to_string())),
        "3.2" => day03::star2(&filename.unwrap_or("inputs/day03.txt".to_string())),
        "4.1" => day04::star1(&filename.unwrap_or("inputs/day04.txt".to_string())),
        "4.2" => day04::star2(&filename.unwrap_or("inputs/day04.txt".to_string())),
        "5.1" => day05::star1(&filename.unwrap_or("inputs/day05.txt".to_string())),
        "5.2" => day05::star2(&filename.unwrap_or("inputs/day05.txt".to_string())),
        _ => println!("Part {} not available, expected '2.1' or similar.", day)
    }
}
