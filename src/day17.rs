use std::fs;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Left,
    Right
}

#[derive(Debug)]
struct Tetris {
    state: Vec<u16>,
    parts: Vec<Vec<u16>>,
    part_index: usize,
    moves: Vec<Move>,
    move_index: usize,
    top: usize,
    max_drop: usize
}


#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    tail: Vec<u16>,
    move_index: usize
}

impl Tetris {
    fn new(moves: Vec<Move>) -> Tetris {
        let parts = vec![vec![120],
                         vec![16,56,16],
                         vec![56,32,32],
                         vec![8,8,8,8],
                         vec![24,24]];
        return Tetris {
            state: Vec::new(),
            parts: parts,
            part_index: 0,
            moves: moves,
            move_index: 0,
            top: 0,
            max_drop: 0
        }
    }

    fn at(&self, i: usize, len: usize) -> Vec<u16> {
        let mut res = Vec::new();
        let mut j = 0;
        while j < len && i + j < self.state.len() {
            res.push(self.state[i+j]);
            j += 1;
        }
        for _ in 0..len-j {
            res.push(257);
        }
        return res;
    }

    fn fits(&self, part: &Vec<u16>, pos: usize) -> bool {
        let space = self.at(pos, part.len());
        return part.iter().zip(space.iter()).all(|(a, b)| a&b == 0);
    }

    fn insert(&mut self, part: &Vec<u16>, pos: usize) {
        let len = part.len();
        let state_len = self.state.len();
        let mut i = 0;
        while i < len && pos+i < state_len {
            self.state[pos+i] |= part[i];
            i += 1;
        }
        while self.state.len() < pos {
            self.state.push(257);
        }
        while i < len {
            self.state.push(257 | part[i]);
            i += 1;
        }
        self.top = self.state.len();
    }

    fn move_one(&mut self, part: &Vec<u16>, pos: usize) -> Vec<u16> {
        let new_part = match self.moves[self.move_index] {
            Move::Left => part.iter().map(|c| c >> 1).collect(),
            Move::Right => part.iter().map(|c| c << 1).collect()
        };
        self.move_index = (self.move_index + 1) % self.moves.len();        
        if self.fits(&new_part, pos) {
            return new_part;        
        }
        return part.clone();
    }
    
    fn add_part(&mut self) {
        let mut part = self.parts[self.part_index].clone();
        let mut pos = self.top+3;
        part = self.move_one(&part, pos);
        while pos > 0 && self.fits(&part, pos-1) {
            pos -= 1;
            part = self.move_one(&part, pos);
        }
        self.max_drop = max(self.max_drop, self.top+3 - pos);
        self.insert(&part, pos);
        self.part_index = (self.part_index + 1) % self.parts.len();
    }
}

impl State {
    fn new(tetris: &Tetris) -> State {
        let tail_start = tetris.state.len() - tetris.max_drop;
        let tail = tetris.state[tail_start..].to_vec();
        return State {
            tail: tail,
            move_index: tetris.move_index
        }
    }
    
}

fn read_moves(filename: &str) -> Vec<Move> {
    let data = fs::read_to_string(filename).unwrap();
    return data.chars().filter(|c| *c == '<' || *c == '>').map(|c| match c {
        '<' => Move::Left,
        '>' => Move::Right,
        _   => panic!("Unknown move")
    }).collect();
}


pub fn star1(filename: &str) {
    let mut tetris = Tetris::new(read_moves(filename));
    for _ in 0..2022 {
        tetris.add_part();
    }
    println!("Star 1: {}", tetris.top);
}

pub fn star2(filename: &str) {
    let mut drops = 1000000000000;
    let mut tetris = Tetris::new(read_moves(filename)); 
    // Initialization
    let iteration = tetris.parts.len() * tetris.moves.len();
    for _ in 0..iteration {
        tetris.add_part();
    }
    drops -= iteration;
    let mut height = tetris.top;
    // Find repeating
    let mut n = 0;
    let mut states: HashMap<State, usize> = HashMap::new();
    let start_height = tetris.top;
    let repeat = loop {
        let state = State::new(&tetris);
        if let Some(m) = states.get(&state) {
            break n-m;
        } else {
            states.insert(state, n);
            for _ in 0..tetris.parts.len() { tetris.add_part(); }
        }
        n += 1;
    };
    let height_added = tetris.top - start_height;    
    let repeat_count = drops / (repeat * tetris.parts.len());
    drops -= repeat_count * repeat * tetris.parts.len();
    height += height_added * repeat_count;
    // Adding final parts
    let final_parts_start = tetris.top;
    for _ in 0..drops {
        tetris.add_part();
    }
    height += tetris.top - final_parts_start;
    println!("Star 1: {}", height);    
}

