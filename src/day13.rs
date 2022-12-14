use json::{JsonValue, array};
use std::cmp::{Ordering, min};

fn cmp(left: &JsonValue, right: &JsonValue) -> Ordering {
    if left.is_number() && right.is_number() {
        return left.as_i32().unwrap().cmp(&right.as_i32().unwrap());
    } else if left.is_array() && right.is_array() {
        for i in 0..min(left.len(), right.len()) {
            let value_cmp = cmp(&left[i], &right[i]);
            if value_cmp != Ordering::Equal {
                return value_cmp;
            }
        }
        return left.len().cmp(&right.len());
    } else if left.is_number() {
        return cmp(&array!(left.as_i32()), &right);
    } else if right.is_number() {
        return cmp(&left, &array!(right.as_i32()));
    }
    
    return Ordering::Equal;
}

pub fn star1(filename: &str) {
    let mut line_iter = super::utils::read_lines(filename).into_iter();
    let mut result = 0;
    let mut index = 1;
    while let Some(left_line) = line_iter.next() {
        let left = json::parse(&left_line).unwrap();
        let right = json::parse(&line_iter.next().unwrap()).unwrap();
        line_iter.next();
        if cmp(&left, &right) != Ordering::Greater {
            result += index;
        }
        index += 1;
    }
    println!("Star 1: {}", result);
}

pub fn star2(filename: &str) {
    let lines = super::utils::read_lines(filename);
    let div1 = array!([2]);
    let div2 = array!([6]);
    let mut packets = Vec::new();
    for line in lines {
        if let Ok(packet) = json::parse(&line) {
            packets.push(packet);
        }
    }
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|a, b| cmp(a, b));
    let pos1 = packets.iter().position(|x| x == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|x| x == &div2).unwrap() + 1;
    println!("Star 2: {}", pos1 * pos2);
}
