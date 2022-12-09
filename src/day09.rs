use regex::Regex;
use std::collections::HashSet;

#[derive(Eq,PartialEq,Hash,Debug,Clone)]
struct Loc {x: i32, y: i32}

impl Loc {
    pub fn step(&mut self, dir: &str) {
        match dir {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "R" => self.x += 1,
            "L" => self.x -= 1,
            &_ => panic!()
        }
    }

    pub fn move_towards(&mut self, to: &Self) {
        let xdiff = to.x - self.x;
        let ydiff = to.y - self.y;
        if xdiff.abs() > 1 || ydiff.abs() > 1 { 
            *self = Self { x: self.x + xdiff.signum(), y: self.y + ydiff.signum() }
        }
    }
}

pub fn compute_steps(filename: &str, length: usize) -> usize {
    let lines = super::utils::read_lines(&filename);
    let re = Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    let mut rope = vec![Loc { x: 0, y: 0 }; length];
    let mut visited = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            for _ in 0..cap[2].parse::<u32>().unwrap() {
                rope[0].step(&cap[1]);
                for i in 1..length {
                    let mut part = rope[i].clone();
                    part.move_towards(&rope[i-1]);
                    rope[i] = part;
                }
                visited.insert(rope[length-1].clone());
            }
        }
    }
    return visited.len();
}

pub fn star1(filename: &str) {
    println!("Star 1: {}", compute_steps(filename, 2));
}

pub fn star2(filename: &str) {
    println!("Star 2: {}", compute_steps(filename, 10));
}
