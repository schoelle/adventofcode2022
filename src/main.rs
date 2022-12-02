mod day01;

use std::env;

fn main() {
    let day = env::args().nth(1).unwrap().parse::<i32>().unwrap();
    let filename = env::args().nth(2).unwrap();
    if day == 1 {
        day01::star1(&filename);
        day01::star2(&filename);
    }
}
