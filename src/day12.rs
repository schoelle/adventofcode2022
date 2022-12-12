use std::collections::VecDeque;
use std::collections::HashSet;
use array2d::Array2D;

type StepCheck = fn(i32, i32) -> bool;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize,usize);

#[derive(Debug)]
struct State { pos: Position, dist: u32 }

#[derive(Debug)]
struct HillMap {
    start: Position,
    end: Vec<Position>,
    map: Array2D<i32>,
    step_check: StepCheck
}

impl HillMap {

    fn new(lines: &Vec<String>, start: Position, end: Vec<Position>, step_check: StepCheck ) -> HillMap {
        return HillMap  {
            map: Array2D::from_rows(&lines.iter().map(
                |l| l.chars().map(|c| match c { 'S' => 'a', 'E' => 'z', _ => c } as i32 - 'a' as i32).
                    collect::<Vec<i32>>()
            ).collect::<Vec<Vec<i32>>>()),
            start,
            end,
            step_check
        };
    }
        
    fn height(&self, pos: &Position) -> i32 { return self.map[(pos.1, pos.0)]; }
    
    fn reachable(&self, pos: &Position) -> Vec<Position> {
        let mut result = Vec::new();
        if pos.0 > 0 { result.push(Position(pos.0 - 1, pos.1)) }
        if pos.1 > 0 { result.push(Position(pos.0, pos.1 - 1)) }
        if pos.0 < self.map.num_columns() - 1 { result.push(Position(pos.0 + 1, pos.1)) }
        if pos.1 < self.map.num_rows() - 1 { result.push(Position(pos.0, pos.1 + 1)) }
        return result.into_iter().filter(|p| (self.step_check)(self.height(pos), self.height(p))).collect();
    }

    fn distance(&self) -> u32 {
        let mut work = VecDeque::new();
        let mut visited: HashSet<Position> = HashSet::new();
        work.push_back(State { pos: self.start, dist: 0 });
        while let Some(v) = work.pop_front() {
            if visited.contains(&v.pos) { continue; }
            visited.insert(v.pos);
            if self.end.contains(&v.pos) { return v.dist }
            self.reachable(&v.pos).into_iter().for_each(
                |e| work.push_back(State { pos: e, dist: v.dist + 1 }
            ));
        }
        panic!("No path found");
    }
}

fn locate(lines: &Vec<String>, needles: HashSet<char>) -> Vec<Position> {
    let mut result = Vec::new();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            if needles.contains(&c) {
                result.push(Position (x, y));
            }
            x += 1;
        }
        y += 1;
    }
    return result;
}

pub fn star1(filename: &str) {
    let lines = super::utils::read_lines(filename);
    let map = HillMap::new(&lines,
                           locate(&lines, HashSet::from(['S']))[0],
                           locate(&lines, HashSet::from(['E'])),
                           |from, to| to - from <= 1);
    println!("Star 1: {:?}", &map.distance());
}

pub fn star2(filename: &str) {
    let lines = super::utils::read_lines(filename);
    let map = HillMap::new(&lines,
                           locate(&lines, HashSet::from(['E']))[0],
                           locate(&lines, HashSet::from(['S', 'a'])),
                           |from, to| from - to <= 1);
    println!("Star 2: {}", &map.distance());
}
