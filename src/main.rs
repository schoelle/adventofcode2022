mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        "6.1" => day06::star1(&filename.unwrap_or("inputs/day06.txt".to_string())),
        "6.2" => day06::star2(&filename.unwrap_or("inputs/day06.txt".to_string())),
        "7.1" => day07::star1(&filename.unwrap_or("inputs/day07.txt".to_string())),
        "7.2" => day07::star2(&filename.unwrap_or("inputs/day07.txt".to_string())),
        "8.1" => day08::star1(&filename.unwrap_or("inputs/day08.txt".to_string())),
        "8.2" => day08::star2(&filename.unwrap_or("inputs/day08.txt".to_string())),
        "9.1" => day09::star1(&filename.unwrap_or("inputs/day09.txt".to_string())),
        "9.2" => day09::star2(&filename.unwrap_or("inputs/day09.txt".to_string())),
        "10.1" => day10::star1(&filename.unwrap_or("inputs/day10.txt".to_string())),
        "10.2" => day10::star2(&filename.unwrap_or("inputs/day10.txt".to_string())),
        "11.1" => day11::star1(&filename.unwrap_or("inputs/day11.txt".to_string())),
        "11.2" => day11::star2(&filename.unwrap_or("inputs/day11.txt".to_string())),
        "12.1" => day12::star1(&filename.unwrap_or("inputs/day12.txt".to_string())),
        "12.2" => day12::star2(&filename.unwrap_or("inputs/day12.txt".to_string())),
        "13.1" => day13::star1(&filename.unwrap_or("inputs/day13.txt".to_string())),
        "13.2" => day13::star2(&filename.unwrap_or("inputs/day13.txt".to_string())),
        "14.1" => day14::star1(&filename.unwrap_or("inputs/day14.txt".to_string())),
        "14.2" => day14::star2(&filename.unwrap_or("inputs/day14.txt".to_string())),
        "15.1" => day15::star1(&filename.unwrap_or("inputs/day15.txt".to_string())),
        "15.2" => day15::star2(&filename.unwrap_or("inputs/day15.txt".to_string())),
        "16.1" => day16::star1(&filename.unwrap_or("inputs/day16.txt".to_string())),
        "16.2" => day16::star2(&filename.unwrap_or("inputs/day16.txt".to_string())),
        "17.1" => day17::star1(&filename.unwrap_or("inputs/day17.txt".to_string())),
        "17.2" => day17::star2(&filename.unwrap_or("inputs/day17.txt".to_string())),
        "18.1" => day18::star1(&filename.unwrap_or("inputs/day18.txt".to_string())),
        "18.2" => day18::star2(&filename.unwrap_or("inputs/day18.txt".to_string())),
        "19.1" => day19::star1(&filename.unwrap_or("inputs/day19.txt".to_string())),
        "19.2" => day19::star2(&filename.unwrap_or("inputs/day19.txt".to_string())),
        "20.1" => day20::star1(&filename.unwrap_or("inputs/day20.txt".to_string())),
        "20.2" => day20::star2(&filename.unwrap_or("inputs/day20.txt".to_string())),
        "21.1" => day21::star1(&filename.unwrap_or("inputs/day21.txt".to_string())),
        "21.2" => day21::star2(&filename.unwrap_or("inputs/day21.txt".to_string())),
        "22.1" => day22::star1(&filename.unwrap_or("inputs/day22.txt".to_string())),
        "22.2" => day22::star2(&filename.unwrap_or("inputs/day22.txt".to_string())),
        "23.1" => day23::star1(&filename.unwrap_or("inputs/day23.txt".to_string())),
        "23.2" => day23::star2(&filename.unwrap_or("inputs/day23.txt".to_string())),
        "24.1" => day24::star1(&filename.unwrap_or("inputs/day24.txt".to_string())),
        "24.2" => day24::star2(&filename.unwrap_or("inputs/day24.txt".to_string())),
        "25.1" => day25::star1(&filename.unwrap_or("inputs/day25.txt".to_string())),
        "25.2" => day25::star2(&filename.unwrap_or("inputs/day25.txt".to_string())),
        _ => println!("Part {} not available, expected '1.1' or similar.", day)
    }
}
