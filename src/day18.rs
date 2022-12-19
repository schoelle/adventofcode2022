use regex::Regex;
use std::collections::HashSet;
use std::cmp::{max, min};

#[derive(Clone,Copy,Hash,PartialEq,Eq,Debug)]
struct Point { x:i32, y:i32, z:i32 }

impl Point {

    fn neighbours(&self) -> HashSet<Point> {
        return HashSet::from([
            Point { x: self.x - 1, ..*self }, Point { x: self.x + 1, ..*self },
            Point { y: self.y - 1, ..*self }, Point { y: self.y + 1, ..*self },
            Point { z: self.z - 1, ..*self }, Point { z: self.z + 1, ..*self }
        ]);
    }

    fn sides_touching(&self, other: &HashSet<Point>) -> i32 {
        return self.neighbours().iter().filter(|p| other.contains(&p)).count() as i32;
    }
}

#[derive(Debug)]
struct BoundingBox { maxx: i32, minx: i32, maxy: i32, miny: i32, maxz: i32, minz: i32 }

impl BoundingBox {
    fn new(points: &HashSet<Point>) -> BoundingBox {
        return BoundingBox {
            maxx: points.iter().fold(i32::MIN, |acc, a| max(acc, a.x)) + 1,
            minx: points.iter().fold(i32::MAX, |acc, a| min(acc, a.x)) - 1,
            maxy: points.iter().fold(i32::MIN, |acc, a| max(acc, a.y)) + 1,
            miny: points.iter().fold(i32::MAX, |acc, a| min(acc, a.y)) - 1,
            maxz: points.iter().fold(i32::MIN, |acc, a| max(acc, a.z)) + 1,
            minz: points.iter().fold(i32::MAX, |acc, a| min(acc, a.z)) - 1
        }
    }

    fn inside(&self, p: Point) -> bool {
        return p.x >= self.minx && p.x <= self.maxx &&
            p.y >= self.miny && p.y <= self.maxy &&
            p.z >= self.minz && p.z <= self.maxz;            
    }
    
    fn outside_points(&self, solid: &HashSet<Point>) -> HashSet<Point> {
        let mut res = HashSet::new();
        let mut work = Vec::from([Point { x: self.minx, y: self.miny, z: self.minz }]);
        while let Some(p) = work.pop() {
            if !solid.contains(&p) && !res.contains(&p) && self.inside(p) {
                res.insert(p);
                p.neighbours().iter().for_each(|n| work.push(*n));
            }            
        }
        return res;
    }
}

fn read_points(filename: &str) -> HashSet<Point> {
    let re = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    let mut result = HashSet::new();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            result.insert(Point{ x: cap[1].parse().unwrap(),
                                 y: cap[2].parse().unwrap(),
                                 z: cap[3].parse().unwrap()});
        }
    }
    return result;
}

pub fn star1(filename: &str) {
    let points = read_points(filename);
    let res: i32 = points.iter().map(|p| 6 - p.sides_touching(&points)).sum();
    println!("Star 1: {}", res);
}

pub fn star2(filename: &str) {
    let points = read_points(filename);
    let outside = BoundingBox::new(&points).outside_points(&points);
    let res: i32 = points.iter().map(|p| p.sides_touching(&outside)).sum();
    println!("Star 2: {}", res);
}
