use array2d::Array2D;

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Pos(usize,usize);

impl Pos {
    fn neighbours(&self, width: usize, height: usize) -> Vec<Pos> {
        let mut res = Vec::new();
        res.push(*self);
        if self.0 > 0 { res.push(Pos(self.0-1, self.1)) }
        if self.1 > 0 { res.push(Pos(self.0, self.1-1)) }
        if self.0 < width-1 { res.push(Pos(self.0+1, self.1)) }
        if self.1 < height-1 { res.push(Pos(self.0, self.1+1)) }
        return res;
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
enum Wind { Up, Down, Left, Right }

impl Wind {
    fn from_char(c: char) -> Wind {
        match c {
            '>' => Wind::Right,
            '<' => Wind::Left,
            '^' => Wind::Up,
            'v' => Wind::Down,
            _ => panic!("Unknown Windection")
        }
    }

    fn step(&self, pos: Pos, width: usize, height: usize) -> Pos {
        match self {
            Wind::Right => if pos.0 == width-2 { Pos(1, pos.1) } else { Pos(pos.0+1, pos.1) },
            Wind::Left => if pos.0 == 1 { Pos(width-2, pos.1) } else { Pos(pos.0-1, pos.1) },
            Wind::Down => if pos.1 == height-2 { Pos(pos.0, 1) } else { Pos(pos.0, pos.1+1) },
            Wind::Up => if pos.1 == 1 { Pos(pos.0, height-2) } else { Pos(pos.0, pos.1-1) },
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
enum Content { None, Wall, Elf, Blizzard(Vec<Wind>) }

impl Content {
    fn from_char(c: char) -> Content {
        match c {
            '>' | '<' | '^' | 'v' => Content::Blizzard(vec![Wind::from_char(c)]),
            '.' => Content::None,
            '#' => Content::Wall,
            _ => panic!("Unknown Content")
        }
    }
}

#[derive(Debug,Clone)]
struct Map { time: u64, height: usize, width: usize, content: Array2D<Content> }

impl Map {
    
    fn read(filename: &str) -> Map {
        let lines = super::utils::read_lines(filename);
        let width = lines[0].len();
        let height = lines.len();
        let mut content = Array2D::filled_with(Content::None, height, width);
        let mut y = 0;
        for line in lines {
            let mut x = 0;
            for c in line.chars() {
            let sp = Content::from_char(c);
                content[(y,x)] = sp;
                x += 1;
        }
            y += 1;
        }
        return Map { time: 0, content, height, width };
    }

    fn step(&mut self) {
        let mut newmap = Array2D::filled_with(Content::None, self.height, self.width);
        let mut elves = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.content[(y,x)] {
                    Content::Wall => newmap[(y,x)] = Content::Wall,
                    Content::Blizzard(bs) => {
                        for b in bs {
                            let newpos = b.step(Pos(x,y), self.width, self.height);
                            if let Content::Blizzard(v) = &mut newmap[(newpos.1, newpos.0)] {
                                v.push(b.clone());
                            } else {
                                newmap[(newpos.1, newpos.0)] = Content::Blizzard(vec![b.clone()]);
                            }
                        }
                    },
                    Content::Elf => {
                        let mut e = Pos(x,y).neighbours(self.width, self.height);
                        elves.append(&mut e);
                    },
                    _ => {}
                }
            }
        };
        for e in elves {
            if newmap[(e.1, e.0)] == Content::None {
                newmap[(e.1, e.0)] = Content::Elf
            }
        }
        self.content = newmap;
        self.time += 1;
    }

    fn set_elf(&mut self, pos: Pos) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.content[(y,x)] == Content::Elf {
                    self.content[(y,x)] = Content::None
                }
            }
        }
        self.content[(pos.1, pos.0)] = Content::Elf;
    }
}

pub fn star1(filename: &str) {
    let mut map = Map::read(filename);
    map.set_elf(Pos(1,0));
    while map.content[(map.height-1, map.width-2)] == Content::None {
        map.step();
    }
    println!("Star 1: {:?}", map.time);
}

pub fn star2(filename: &str) {
    let mut map = Map::read(filename);
    map.set_elf(Pos(1,0));
    while map.content[(map.height-1, map.width-2)] == Content::None {
        map.step();
    }
    map.set_elf(Pos(map.width-2,map.height-1));
    while map.content[(0, 1)] == Content::None {
        map.step();
    }
    map.set_elf(Pos(1,0));
    while map.content[(map.height-1, map.width-2)] == Content::None {
        map.step();
    }
    println!("Star 2: {}", map.time);
}
