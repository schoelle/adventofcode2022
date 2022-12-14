use std::collections::HashSet;
use regex::Regex;
use std::cmp::{max, min};

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Loc(u32,u32);

#[derive(Debug)]
struct Cave {
    // Ok - a bit cheap, but saves me figuring out dimensions
    occ: HashSet<Loc>,
    depth: u32
}

impl Cave {
    fn new() -> Cave {
        return Cave { occ: HashSet::new(), depth: 0 }
    }
    
    fn add_line(&mut self, from: Loc, to: Loc) {
        if from.0 == to.0 { // Vertical
            self.depth = max(self.depth, max(from.1, to.1));
            for i in min(from.1, to.1)..=max(from.1, to.1) {
                self.occ.insert(Loc(from.0, i));
            }
        } else if from.1 == to.1 { // Horizontal
            self.depth = max(self.depth, from.1);
            for i in min(from.0, to.0)..=max(from.0, to.0) {
                self.occ.insert(Loc(i, from.1));
            }
        } else {
            panic!("Only horizontal and vertical lines supported");
        }
    }

    fn add_sand(&mut self, start: Loc, floor: bool) -> bool {
        let mut pos = start;
        if self.occ.contains(&pos) {
            return false;
        }
        while pos.1 <= self.depth + 2 {
            let x = pos.0;
            let y = pos.1;
            if floor && pos.1 == self.depth + 1 {
                self.occ.insert(pos);
                return true;                
            } else if !self.occ.contains(&Loc(x, y+1)) {
                pos = Loc(x, y+1);
            } else if !self.occ.contains(&Loc(x-1, y+1)) {
                pos = Loc(x-1, y+1);
            } else if !self.occ.contains(&Loc(x+1, y+1)) {
                pos = Loc(x+1, y+1);
            } else {
                self.occ.insert(pos);
                return true;
            }        
        }
        return false;
    }   
}

fn build_cave(filename: &str) -> Cave {
    let mut result = Cave::new();
    let lines = super::utils::read_lines(filename);
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    for line in lines {
        let mut cap_iter = re.captures_iter(&line);
        let start = cap_iter.next().unwrap();
        let mut from = Loc(start[1].parse().unwrap(), start[2].parse().unwrap());
        for cap in cap_iter {
            let to = Loc(cap[1].parse().unwrap(), cap[2].parse().unwrap());
            result.add_line(from, to);
            from = to;
        }
    }
    return result;
}


pub fn star1(filename: &str) {
    let mut cave = build_cave(filename);
    let mut count = 0;
    while cave.add_sand(Loc(500,0), false) {
        count += 1;
    }
    println!("Star 1: {}", count);
}

pub fn star2(filename: &str) {
    let mut cave = build_cave(filename);
    let mut count = 0;
    while cave.add_sand(Loc(500,0), true) {
        count += 1;
    }
    println!("Star 2: {}", count);
}
