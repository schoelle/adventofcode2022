use regex::Regex;

struct Move {
    count: usize,
    from: usize,
    to: usize
}

struct State {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>
}

impl State {
    fn apply_one_move(&mut self, one_by_one: bool) {
        let a_move = self.moves.pop().unwrap();
        let from_stack = &self.stacks[(a_move.from-1)];
        let to_stack = &self.stacks[(a_move.to-1)];
        let (front, rest) = from_stack.split_at(a_move.count);
        let mut new_stack = front.to_vec();
        if one_by_one {
            new_stack.reverse();
        }
        new_stack.extend_from_slice(to_stack.as_ref());
        self.stacks[a_move.from-1] = rest.to_vec();
        self.stacks[a_move.to-1] = new_stack;
    }

    fn apply_all_moves(&mut self, one_by_one: bool) {
        while !self.moves.is_empty() {
            self.apply_one_move(one_by_one);
        }
    }

    fn top_of_stacks(&self) -> String {
        return String::from_iter(self.stacks.iter().map(|s| s[0]))
    }
}

fn build_input(filename: &str) -> State {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = super::utils::read_lines(filename);
    for line in &mut lines {
        if !line.contains("[") {
            break;
        }
        let mut chars = line.chars();
        let mut pos = 0;
        chars.next();
        while let Some(c) = chars.next() {
            chars.next();
            chars.next();
            chars.next();
            if c != ' ' {
                while stacks.len() <= pos {
                    stacks.push(Vec::new());
                }
                stacks[pos].push(c);
            }
            pos += 1;
        }
    }
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut moves = Vec::new();
    for line in &mut lines {
        if let Some(cap) = re.captures(&line) {
            let a_move = Move { count: cap[1].parse().unwrap(),
                                from: cap[2].parse().unwrap(),
                                to: cap[3].parse().unwrap() };
            moves.push(a_move);
        }
    }
    moves.reverse();
    return State { stacks: stacks, moves: moves };
}

pub fn star1(filename: &str) {
    let mut state = build_input(filename);
    state.apply_all_moves(true);
    println!("Star 1: {}", state.top_of_stacks());
}

pub fn star2(filename: &str) {
    let mut state = build_input(filename);
    state.apply_all_moves(false);
    println!("Star 2: {}", state.top_of_stacks());
}
