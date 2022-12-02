mod utils;
mod day01;
mod day02;

use std::env;

fn main() {
    let day = env::args().nth(1).unwrap();
    let filename = env::args().nth(2);
    match day.as_ref() {
        "1.1" => day01::star1(&filename.unwrap_or("inputs/day01.txt".to_string())),
        "1.2" => day01::star2(&filename.unwrap_or("inputs/day01.txt".to_string())),
        "2.1" => day02::star1(&filename.unwrap_or("inputs/day02.txt".to_string())),
        "2.2" => day02::star2(&filename.unwrap_or("inputs/day02.txt".to_string())),
        _ => println!("Part {} not available, expected '2.1' or similar.", day)
    }
}
