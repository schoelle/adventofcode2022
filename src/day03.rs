fn char_value(v: char) -> u32 {
    let value = v as u32;
    if value > 96 {
        return value - 96;
    }
    return value - 38;
}

fn wrong_item(line: String) -> u32 {
    let mut chars: [u32; 53] = [0; 53];
    let length = line.len() / 2;
    let front = &line[..length];
    let back = &line[length..];
    for c in front.chars() {
        let value = char_value(c);
        chars[value as usize] = 1;
    }
    for c in back.chars() {
        let value = char_value(c);
        if chars[value as usize] == 1 {
            return value;
        }
    }
    return 0;
}

fn find_badge(first: &String, second: &String, third: &String) -> u32 {
    let mut chars: [u32; 53] = [0; 53];
    for c in first.chars() {
        let value = char_value(c);
        chars[value as usize] = 1;
    }
    for c in second.chars() {
        let value = char_value(c);
        if chars[value as usize] == 1 {
            chars[value as usize] = 2;
        }
    }
    for c in third.chars() {
        let value = char_value(c);
        if chars[value as usize] == 2 {
            return value;
        }
    }
    return 0;
}

pub fn star1(filename: &str) {
    let mut total = 0;
    for line in super::utils::read_lines(filename) {
        total += wrong_item(line);
    }
    println!("Star 1: {}", total);
}

pub fn star2(filename: &str) {
    let mut total = 0;
    let lines = super::utils::read_lines(filename);
    let mut iter = lines.iter();
    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        total += find_badge(first, second, third);
    }    
    println!("Star 2: {}", total);
}
