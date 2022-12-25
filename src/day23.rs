use std::collections::{HashSet,HashMap};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Pos(i64,i64);

#[derive(Debug)]
struct Elf {
    pos: Pos,
    target: Pos
}

fn plan_step(elves: &mut Vec<Elf>, round: u64) -> bool {
    let elfloc: HashSet<Pos> = HashSet::from_iter(elves.iter().map(|e| e.pos));
    let mut change = false;
    for mut elf in elves {
        elf.target = elf.pos.clone(); // Consider doing nothing first
        let n = !elfloc.contains(&Pos(elf.pos.0, elf.pos.1-1));
        let ne = !elfloc.contains(&Pos(elf.pos.0+1, elf.pos.1-1));
        let e = !elfloc.contains(&Pos(elf.pos.0+1, elf.pos.1));
        let se = !elfloc.contains(&Pos(elf.pos.0+1, elf.pos.1+1));
        let s = !elfloc.contains(&Pos(elf.pos.0, elf.pos.1+1));
        let sw = !elfloc.contains(&Pos(elf.pos.0-1, elf.pos.1+1));
        let w = !elfloc.contains(&Pos(elf.pos.0-1, elf.pos.1));
        let nw = !elfloc.contains(&Pos(elf.pos.0-1, elf.pos.1-1));
        if n && ne && e && se && s && sw && w && nw {
            continue;
        }
        let dir = vec![
            (nw && n && ne, Pos(elf.pos.0, elf.pos.1-1)),
            (sw && s && se, Pos(elf.pos.0, elf.pos.1+1)),
            (nw && w && sw, Pos(elf.pos.0-1, elf.pos.1)),
            (ne && e && se, Pos(elf.pos.0+1, elf.pos.1)),
        ];
        for i in 0..4 {
            let (b, p) = dir[((i+round) % 4) as usize];
            if b {
                change = true;
                elf.target = p;
                break;
            }
        }
    }
    return change;
}

fn box_size(elves: &Vec<Elf>) -> i64 {
    let min_x = elves.iter().map(|e| e.pos.0).min().unwrap();
    let min_y = elves.iter().map(|e| e.pos.1).min().unwrap();
    let max_x = elves.iter().map(|e| e.pos.0).max().unwrap();
    let max_y = elves.iter().map(|e| e.pos.1).max().unwrap();
    return ((max_x-min_x+1) * (max_y-min_y+1)) - elves.len() as i64;
}

fn do_step(elves: &mut Vec<Elf>) {
    let mut targets: HashMap<Pos,u64> = HashMap::new();
    elves.iter().for_each(|e| *targets.entry(e.target).or_insert(0) +=1 );
    for mut e in elves {
        if targets[&e.target] == 1 { e.pos = e.target };
    }
}

fn read_elves(filename: &str) -> Vec<Elf> {
    let mut res = Vec::new();
    let mut y = 0;
    for line in super::utils::read_lines(filename) {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                res.push(Elf { pos: Pos(x,y), target: Pos(x,y) });
            }
            x += 1;
        }
        y += 1;
    }
    return res;
}

pub fn star1(filename: &str) {
    let mut elves = read_elves(filename);
    for round in 0..10 {
        plan_step(&mut elves, round);
        do_step(&mut elves);
    }
    println!("Star 1: {:?}", box_size(&elves));
}

pub fn star2(filename: &str) {
    let mut elves = read_elves(filename);
    let mut round = 0;
    while plan_step(&mut elves, round) {
        do_step(&mut elves);
        round += 1;
    }
    println!("Star 2: {}", round+1);
}
