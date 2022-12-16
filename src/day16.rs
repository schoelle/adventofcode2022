use std::collections::HashMap;
use regex::Regex;
use array2d::Array2D;
use std::cmp::min;

#[derive(Debug)]
struct ValveDefinition {
    name: String,
    rate: i32,
    exits: Vec<String>
}

#[derive(Debug)]
struct Maze {
    start: usize,
    size: usize,
    rate: HashMap<usize, i32>,
    dist: Array2D<i32>
}

impl Maze {
    fn new(defs: Vec<ValveDefinition>) -> Maze {
        let mut big_start: usize = usize::MAX;
        let mut big_index: HashMap<String, usize> = HashMap::new();
        let mut big_rate: HashMap<usize, i32> = HashMap::new();
        for (i, e) in defs.iter().enumerate() {
            big_index.insert(e.name.clone(), i);
            big_rate.insert(i, e.rate);
            if e.name == "AA" { big_start = i; }
        }
        let big_size = big_index.len();
        let mut big_dist = Array2D::filled_with(99, big_index.len(), big_index.len());
        for def in &defs {
            big_dist[(big_index[&def.name], big_index[&def.name])] = 0;
            for exit in &def.exits {
                big_dist[(big_index[&def.name], big_index[exit])] = 1;                
            }
        }
        // Floyd-Warshall
        for a in 0..big_index.len() {
            for b in 0..big_index.len() {
                for c in 0..big_index.len() {
                    big_dist[(b,c)] = min(big_dist[(b,c)], big_dist[(b,a)] + big_dist[(a,c)]);
                }
            }
        }
        let interesting: Vec<usize> = (0..big_size).filter( 
            |i| (big_dist[(*i, big_start)] < 99 && big_rate[&i] > 0) || (*i == big_start) 
        ).collect();
        let size = interesting.len();
        let mut dist = Array2D::filled_with(99, interesting.len(), interesting.len());
        let mut rate: HashMap<usize, i32> = HashMap::new();
        let mut start = usize::MAX;
        for y in 0..size {
            for x in 0..size {
                dist[(x,y)] = big_dist[(interesting[x], interesting[y])] + 1;
            }
            rate.insert(y, defs[interesting[y]].rate);
            if defs[interesting[y]].name == "AA" { start = y; }
        }
        return Maze { start, size, rate, dist }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Worker(i32, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    available: Vec<usize>,
    workers: Vec<Worker>
}

fn solve(curr: State, maze: &Maze, cache: &mut HashMap<State, i32>) -> i32 {
    if let Some(v) = cache.get(&curr) {
        return *v;
    }
    let mut best = 0;
    for w in 0..curr.workers.len() {
        let worker = &curr.workers[w];
        for next in curr.available.iter() {
            let new_available = curr.available.iter().filter(|x| *x != next).map(|x| *x).collect::<Vec<usize>>();
            let time_left = worker.0 - maze.dist[(worker.1, *next)];       
            if time_left < 0 {
                continue;
            }
            let mut workers = curr.workers.clone();
            workers[w] = Worker(time_left, *next);
            workers.sort();
            let a_try = State {
                available: new_available,
                workers: workers
            };
            let result = solve(a_try, maze, cache) + (time_left * maze.rate[next]);
            if best < result {
                best = result;
            }
        }
    }
    cache.insert(curr, best);
    return best;
}

fn read_definitions(filename: &str) -> Vec<ValveDefinition> {
    let mut result = Vec::new();
    let re = Regex::new(r"^Valve (..) .* rate=(\d+); .*valves? (.*)$").unwrap();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            let name = cap[1].to_string();
            let rate: i32 = cap[2].parse().unwrap();
            let exits = Vec::from_iter(cap[3].split(", ").map(|s| String::from(s)));
            result.push(ValveDefinition { name, rate, exits });
        }
    }
    return result;
}

pub fn star1(filename: &str) {
    let mut cache = HashMap::new();
    let maze = Maze::new(read_definitions(filename));
    let start_solution = State {
        available: Vec::from_iter(0..maze.size),
        workers: Vec::from([ Worker(30, maze.start) ])
    };
    println!("Star 1: {}", solve(start_solution, &maze, &mut cache));
}

pub fn star2(filename: &str) {
    let mut cache = HashMap::new();
    let maze = Maze::new(read_definitions(filename));
    let start_solution = State {
        available: Vec::from_iter(0..maze.size),
        workers: Vec::from([ Worker(26, maze.start), Worker(26, maze.start) ])
    };
    println!("Star 2: {}", solve(start_solution, &maze, &mut cache));
}
