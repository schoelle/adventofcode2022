use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::cmp::max;

fn main() {
    let filename = env::args().nth(1).unwrap();
    star1(&filename);
    star2(&filename);
}

fn star1(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut max_value = 0;
    let mut total = 0;
    for line in io::BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            total = 0;
        } else {
            total += content.parse::<i32>().unwrap();
        }
        max_value = max(max_value, total);
    }
    println!("Star 1: {}", max_value); 
}

fn star2(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut totals = Vec::new();
    let mut total = 0;
    for line in io::BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            totals.push(total);
            total = 0;
        } else {
            total += content.parse::<i32>().unwrap();
        }
    }
    if total > 0 {
        totals.push(total);
    }
    totals.sort();
    totals.reverse();
    let result: i32 = totals[..3].iter().sum();
    println!("Star 2: {}", result);
}


