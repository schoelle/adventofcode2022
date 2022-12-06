use std::fs;
use std::collections::HashSet;
use std::panic;

fn all_different(part: &str) -> bool {
    let char_set: HashSet<char> = HashSet::from_iter(part.chars());
    return char_set.len() == part.len();
}

fn find_start_of_message(data: &str, count: usize) -> usize {
    let mut idx = count;
    while idx < data.len() {
        let part = &data[idx-count..idx];
        if all_different(part) {
            return idx;
        }
        idx += 1;
    }
    panic!("Marker not found!");
}

pub fn star1(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    println!("Star 1: {}", find_start_of_message(&data, 4));
}

pub fn star2(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    println!("Star 2: {}", find_start_of_message(&data, 14));
}
