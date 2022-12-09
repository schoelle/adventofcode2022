use regex::Regex;
use std::collections::HashSet;

#[derive(Eq,PartialEq,Hash,Debug,Clone)]
struct Loc {x: i32, y: i32}

impl Loc {
    
    pub fn new() -> Self {
        Self {x: 0, y: 0}
    }

    pub fn step(&mut self, dir: &str) {
        match dir {
            "U" => *self = Self { x: self.x, y: self.y + 1 },
            "D" => *self = Self { x: self.x, y: self.y - 1 },
            "R" => *self = Self { x: self.x + 1, y: self.y },
            "L" => *self = Self { x: self.x - 1, y: self.y },
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

pub fn star1(filename: &str) {
    let lines = super::utils::read_lines(&filename);
    let re = Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    let mut head = Loc::new();
    let mut tail = Loc::new();
    let mut visited = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            for _ in 0..cap[2].parse::<u32>().unwrap() {
                head.step(&cap[1]);
                tail.move_towards(&head);
                visited.insert(tail.clone());
            }
        }
    }
    println!("Star 1: {}", visited.len());
}

pub fn star2(filename: &str) {
    let lines = super::utils::read_lines(&filename);
    let re = Regex::new(r"^([UDLR]) (\d+)$").unwrap();
    let mut rope = vec![Loc::new(); 10];
    let mut visited = HashSet::new();
    for line in lines {
        if let Some(cap) = re.captures(&line) {
            for _ in 0..cap[2].parse::<u32>().unwrap() {
                let mut head = rope[0].clone();
                head.step(&cap[1]);
                rope[0] = head;
                for i in 1..10 {
                    let mut part = rope[i].clone();
                    part.move_towards(&rope[i-1]);
                    rope[i] = part;
                }
                visited.insert(rope[9].clone());
            }
        }
    }
    println!("Star 2: {}", visited.len());
}
