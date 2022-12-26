use array2d::Array2D;
use gcd::binary_u64;
use regex::Regex;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Pos(usize,usize);

impl Pos {
    fn pos_mod(&self, by_x: i64, by_y: i64) -> Pos {
        let new_x = ((((self.0 as i64 + by_x) % 5) + 5) % 5) as usize;
        let new_y = ((((self.1 as i64 + by_y) % 5) + 5) % 5) as usize;
        Pos(new_x, new_y)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Content { Open, Solid }

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn from_usize(d: usize) -> Dir {
        match d {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("Unknown directon")
        }
    }

    fn idx(&self) -> usize {
        match self {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3
        }
    }
    
    fn turn_right(&self) -> Dir {
        Dir::from_usize((self.idx() + 1) % 4)
    }

    fn turn_left(&self) -> Dir {
        Dir::from_usize((self.idx() + 3) % 4)
    }
 
    fn enter_map(&self, offset: usize, size: usize) -> Pos {
        match self {
            Dir::Up => Pos(offset, size-1),
            Dir::Right => Pos(0, offset),
            Dir::Down => Pos(size-offset-1, 0),
            Dir::Left => Pos(size-1, size-offset-1)
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Exit(Pos,Dir);

#[derive(Debug,Clone)]
struct Map {
    content: Array2D<Content>,
    pos: Pos,
    exits: Vec<Option<Exit>>
}

impl Map {
    fn new(content: Array2D<Content>, pos: Pos) -> Map {
        Map { content, pos, exits: vec![None, None, None, None] }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Cursor {
    dir: Dir,
    map: Pos,
    inner: Pos
}

impl Cursor {
    fn password(&self, size: usize) -> usize {
        let x = self.map.0 * size + self.inner.0 + 1;
        let y = self.map.1 * size + self.inner.1 + 1;
        let dir = self.dir.idx();
        return y * 1000 + x * 4 + dir;
    }
}

#[derive(Debug,Clone)]
enum Instruction {
    Move(u64),
    TurnLeft,
    TurnRight
}

#[derive(Debug)]
struct MapConnection {
    maps: Vec<Map>,
    size: usize,
    cursor: Option<Cursor>,
    instructions: Vec<Instruction>
}

impl MapConnection {
    fn read(filename: &str) -> MapConnection {
        let all_height = super::utils::read_lines(filename).iter().
            position(|l| l.is_empty()).unwrap();
        let all_width = super::utils::read_lines(filename).iter().
            take(all_height as usize).map(|l| l.len()).max().unwrap();
        let size = binary_u64(all_width as u64, all_height as u64) as usize;
        let mut res = MapConnection{ maps: Vec::new(), size, cursor: None,
                                     instructions: Vec::new() };
        let mut y = 0;
        let lines = super::utils::read_lines(filename);
        let mut line_iter = lines.iter();
        let re = Regex::new(r"(R|L|\d+)").unwrap();
        while let Some(line) = line_iter.next() {
            if line.is_empty() {
                if let Some(line) = line_iter.next() {
                    for cap in re.captures_iter(line) {
                        match &cap[1] {
                            "R" => res.instructions.push(Instruction::TurnRight),
                            "L" => res.instructions.push(Instruction::TurnLeft),
                            v => res.instructions.push(Instruction::Move(v.parse::<u64>().unwrap()))
                        }
                    }
                }
                break;
            }
            let mut x = 0;
            for c in line.chars() {
                match c {
                    '.' => res.put(Content::Open, Pos(x,y)),
                    '#' => res.put(Content::Solid, Pos(x,y)),
                    _ => {}
                }
                x += 1;
            }
            y += 1;
        }
        return res;
    }

    fn put(&mut self, value: Content, pos: Pos) {
        let map_pos = Pos(pos.0 / self.size, pos.1 / self.size);
        let inner_pos = Pos(pos.0 % self.size, pos.1 % self.size);
        if let Some(c) = self.maps.iter_mut().find(|x| x.pos == map_pos) {
            c.content[(inner_pos.1, inner_pos.0)] = value;
        } else {
            let mut c = Map::new(Array2D::filled_with(Content::Open, self.size, self.size),
                                 map_pos);
            c.content[(inner_pos.1, inner_pos.0)] = value;            
            self.maps.push(c);
        };
        if self.cursor.is_none() && value == Content::Open {
            self.cursor = Some(Cursor { dir: Dir::Right, map: map_pos, inner: inner_pos });
        }
    }

    fn get(&self, map: Pos, inner: Pos) -> Content {
        let m = self.maps.iter().find(|x| x.pos == map).unwrap();
        m.content[(inner.1, inner.0)]
    }

    fn find_part1_exits(&mut self) {
        let maps = self.maps.clone();
        for map in self.maps.iter_mut() {
            let mut i = -1;
            while maps.iter().all(|m| m.pos != map.pos.pos_mod(i,0)) { i -= 1; }
            map.exits[Dir::Left.idx()] =
                Some(Exit(map.pos.pos_mod(i,0), Dir::Left));
            i = 1;
            while maps.iter().all(|m| m.pos != map.pos.pos_mod(i,0)) { i += 1; }
            map.exits[Dir::Right.idx()] =
                Some(Exit(map.pos.pos_mod(i,0), Dir::Right));
            i = -1;
            while maps.iter().all(|m| m.pos != map.pos.pos_mod(0,i)) { i -= 1; }
            map.exits[Dir::Up.idx()] =
                Some(Exit(map.pos.pos_mod(0,i), Dir::Up));
            i = 1;
            while maps.iter().all(|m| m.pos != map.pos.pos_mod(0,i)) { i += 1; }
            map.exits[Dir::Down.idx()] =
                Some(Exit(map.pos.pos_mod(0,i), Dir::Down));
        }
    }

    fn find_part2_exits(&mut self) {
        let maps = self.maps.clone();
        for map in self.maps.iter_mut() {
            if let Some(n) = maps.iter().find(|m| m.pos == map.pos.pos_mod(-1,0)) {
                map.exits[Dir::Left.idx()] = Some(Exit(n.pos, Dir::Left));
            }
            if let Some(n) = maps.iter().find(|m| m.pos == map.pos.pos_mod(1,0)) {
                map.exits[Dir::Right.idx()] = Some(Exit(n.pos, Dir::Right));
            }
            if let Some(n) = maps.iter().find(|m| m.pos == map.pos.pos_mod(0,-1)) {
                map.exits[Dir::Up.idx()] = Some(Exit(n.pos, Dir::Up));
            }
            if let Some(n) = maps.iter().find(|m| m.pos == map.pos.pos_mod(0,1)) {
                map.exits[Dir::Down.idx()] = Some(Exit(n.pos, Dir::Down));
            }
        }
        loop {
            let maps = self.maps.clone();
            for map in maps {
                for d in 0..4 {
                    if let Some(e1) = map.exits[d] {
                        if let Some(e2) = map.exits[(d+1) %4] {
                            let m1 = self.maps.iter_mut().find(|m| m.pos == e1.0).unwrap();
                            let fd1 = e1.1.turn_right();
                            let td1 = e2.1.turn_right();
                            m1.exits[fd1.idx()] = Some(Exit(e2.0, td1));
                            let m2 = self.maps.iter_mut().find(|m| m.pos == e2.0).unwrap();
                            let fd2 = e2.1.turn_left();
                            let td2 = e1.1.turn_left();
                            m2.exits[fd2.idx()] = Some(Exit(e1.0, td2));
                        }
                    }
                }
            }
            if self.maps.iter().map(|m| m.exits.iter().
                                    filter(|e| e.is_some()).count()).sum::<usize>() == 24 {
                break;
            }
        }
    }
    
    fn step_up(&mut self) {
        let cursor = self.cursor.unwrap();
        let map_pos = cursor.map;
        let inner_pos = cursor.inner;
        let (new_dir, new_map, new_inner) = if inner_pos.1 == 0 {
            let map = self.maps.iter().find(|x| x.pos == map_pos).unwrap();
            let exit = map.exits[Dir::Up.idx()].unwrap();
            (exit.1, exit.0, exit.1.enter_map(inner_pos.0, self.size)) 
        } else {
            (cursor.dir, map_pos, Pos(inner_pos.0, inner_pos.1-1))
        };
        if self.get(new_map, new_inner) == Content::Open {
            self.cursor = Some(Cursor {
                dir: new_dir,
                map: new_map,
                inner: new_inner
            });
        }
    }
    
    fn step_right(&mut self) {
        let cursor = self.cursor.unwrap();
        let map_pos = cursor.map;
        let inner_pos = cursor.inner;
        let (new_dir, new_map, new_inner) = if inner_pos.0 == self.size-1 {
            let map = self.maps.iter().find(|x| x.pos == map_pos).unwrap();
            let exit = map.exits[Dir::Right.idx()].unwrap();
            (exit.1, exit.0, exit.1.enter_map(inner_pos.1, self.size)) 
        } else {
            (cursor.dir, map_pos, Pos(inner_pos.0+1, inner_pos.1))
        };
        if self.get(new_map, new_inner) == Content::Open {
            self.cursor = Some(Cursor {
                dir: new_dir,
                map: new_map,
                inner: new_inner
            });
        }
    }
    
    fn step_down(&mut self) {
        let cursor = self.cursor.unwrap();
        let map_pos = cursor.map;
        let inner_pos = cursor.inner;
        let (new_dir, new_map, new_inner) = if inner_pos.1 == self.size-1 {
            let map = self.maps.iter().find(|x| x.pos == map_pos).unwrap();
            let exit = map.exits[Dir::Down.idx()].unwrap();
            (exit.1, exit.0, exit.1.enter_map(self.size-inner_pos.0-1, self.size)) 
        } else {
            (cursor.dir, map_pos, Pos(inner_pos.0, inner_pos.1+1))
        };
        if self.get(new_map, new_inner) == Content::Open {
            self.cursor = Some(Cursor {
                dir: new_dir,
                map: new_map,
                inner: new_inner
            });
        }        
    }
    
    fn step_left(&mut self) {
        let cursor = self.cursor.unwrap();
        let map_pos = cursor.map;
        let inner_pos = cursor.inner;
        let (new_dir, new_map, new_inner) = if inner_pos.0 == 0 {
            let map = self.maps.iter().find(|x| x.pos == map_pos).unwrap();
            let exit = map.exits[Dir::Left.idx()].unwrap();
            (exit.1, exit.0, exit.1.enter_map(self.size-inner_pos.1-1, self.size)) 
        } else {
            (cursor.dir, map_pos, Pos(inner_pos.0-1, inner_pos.1))
        };
        if self.get(new_map, new_inner) == Content::Open {
            self.cursor = Some(Cursor {
                dir: new_dir,
                map: new_map,
                inner: new_inner
            });
        }                
    }

    fn turn_left(&mut self) {
        let cursor = self.cursor.unwrap();
        self.cursor = Some(Cursor { dir: cursor.dir.turn_left(), ..cursor });        
    }
    
    fn turn_right(&mut self) {
        let cursor = self.cursor.unwrap();
        self.cursor = Some(Cursor { dir: cursor.dir.turn_right(), ..cursor });        
    }
    
    fn step(&mut self) {
        match self.cursor.unwrap().dir {
            Dir::Up => self.step_up(),
            Dir::Right => self.step_right(),
            Dir::Down => self.step_down(),
            Dir::Left => self.step_left(),
        }
    }

    fn execute(&mut self) {
        let instructions = self.instructions.clone();
        for i in instructions.iter() {
            match i {
                Instruction::TurnRight => self.turn_right(),
                Instruction::TurnLeft => self.turn_left(),
                Instruction::Move(n) => (0..*n).for_each(|_| self.step()),
            }
        }
    }
}


pub fn star1(filename: &str) {
    let mut map_connection = MapConnection::read(filename);
    map_connection.find_part1_exits();
    map_connection.execute();
    println!("Star 1: {:?}", map_connection.cursor.unwrap().password(map_connection.size));
}

pub fn star2(filename: &str) {
    let mut map_connection = MapConnection::read(filename);
    map_connection.find_part2_exits();
    map_connection.execute();
    println!("Star 2: {:?}", map_connection.cursor.unwrap().password(map_connection.size));
}
