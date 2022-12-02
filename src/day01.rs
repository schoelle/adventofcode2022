use std::cmp::max;

pub fn star1(filename: &str) {
    let mut max_value = 0;
    let mut total = 0;
    for line in super::utils::read_lines(&filename) {
        if line.is_empty() {
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
        }
        max_value = max(max_value, total);
    }
    println!("Star 1: {}", max_value); 
}

pub fn star2(filename: &str) {
    let mut totals = Vec::new();
    let mut total = 0;
    for line in super::utils::read_lines(&filename) {
        if line.is_empty() {
            totals.push(total);
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap();
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
