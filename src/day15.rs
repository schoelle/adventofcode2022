use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Point(i64, i64);

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Interval(i64, i64);

#[derive(Debug)]
struct IntervalSet { elems: Vec<Interval> }

#[derive(Debug)]
struct Diamond { center: Point, beacon: Point, size: i64 }


impl Point {
    fn distance(&self, other: Point) -> i64 {
        return (self.0 - other.0).abs() + (self.1 - other.1).abs();
    }
}

impl Diamond {
    fn new(center: Point, beacon: Point) -> Diamond {
        Diamond { center: center,
                  beacon: beacon,
                  size: center.distance(beacon) }
    }

    fn interval(&self, y: i64) -> Option<Interval> {
        let y_diff = (y - self.center.1).abs();
        let mut x_start = (self.center.0 - self.size) + y_diff;
        let mut x_end = (self.center.0 + self.size) - y_diff;
        if y == self.beacon.1 {
            if x_start == self.beacon.0 {
                x_start += 1;
            }
            if x_end == self.beacon.0 {
                x_end -= 1;
            }
        }
        if x_start <= x_end {
            return Some(Interval(x_start, x_end));
        }
        return None;            
    }
}

impl Interval {
    fn mergable(&self, other: Interval) -> bool {
        return (self.1 >= other.0 - 1) && (other.1 >= self.0 - 1);
    }

    fn merge(&self, other: Interval) -> Interval {
        return Interval(min(self.0, other.0), max(self.1, other.1));
    }
}

impl IntervalSet {
    fn new() -> IntervalSet {
        return IntervalSet { elems: Vec::new() }
    }

    fn add(&mut self, interval: Interval) {
        let mut current = interval;
        let mut update: Vec<Interval> = Vec::new();
        let elems = &self.elems;
        let mut items = elems.into_iter();
        while let Some(item) = items.next() {
            if item.mergable(current) {
                current = item.merge(current);
            } else if item.0 < current.0 {
                update.push(*item);
            } else {
                update.push(current);
                current = *item;
            }
        }
        update.push(current);
        self.elems = update;
    }

    fn possible_positions(&self) -> i64 {
        return self.elems.iter().map(|x| (x.1 - x.0) + 1).sum::<i64>();
    }

    fn limit(&mut self, min_x: i64, max_x: i64) {
        let mut update: Vec<Interval> = Vec::new();
        let elems = &self.elems;
        let mut items = elems.into_iter();
        while let Some(item) = items.next() {
            if item.1 >= min_x {
                update.push(Interval(max(min_x, item.0), item.1));
                break;
            }
        }
        while let Some(item) = items.next() {
            if item.1 >= max_x {
                update.push(Interval(item.0, min(max_x,item.1)));
                break;
            }
            update.push(*item);
        }
        self.elems = update;
    }

    fn missing(&self) -> Option<i64> {
        if self.elems.len() == 2 && (self.elems[0].1 + 2 == self.elems[1].0) {
            return Some(self.elems[0].1 + 1);
        }
        return None
    }
}


fn read_diamonds(filename: &str) -> Vec<Diamond> {
    let re = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut result = Vec::new();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            result.push(Diamond::new(Point(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                                     Point(cap[3].parse().unwrap(), cap[4].parse().unwrap())));
        }
    }
    return result;
}

pub fn star1(filename: &str) {
    let line = 2000000;
    let diamonds = read_diamonds(filename);
    let mut interval_set = IntervalSet::new();
    for diamond in diamonds {
        if let Some(i) = diamond.interval(line) {
            interval_set.add(i);
        }
    }
    println!("Star 1: {:?}", interval_set.possible_positions());
}

pub fn star2(filename: &str) {
    let limit = 4000000;
    let diamonds = read_diamonds(filename);
    let beacons: HashSet<Point> = HashSet::from_iter(diamonds.iter().map(|d| d.beacon));
    for y in 0..=limit {
        let mut interval_set = IntervalSet::new();
        diamonds.iter().for_each(|d| {
            if let Some(i) = d.interval(y) {
                interval_set.add(i);
            }
        });
        interval_set.limit(0, limit);
        if let Some(x) = interval_set.missing() {
            if !beacons.contains(&Point(x,y)) {
                println!("Star 2: {:?}", x * 4000000 + y);
                break;
            }
        }
    }
}
