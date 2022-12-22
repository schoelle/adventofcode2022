#[derive(Debug)]
struct Shuffle {
    base: Vec<i64>,
    position: Vec<usize>,
    size: usize,
    times: usize
}

// Rust does not have a modulo, just 'remainder', so I have to define my own
fn modop(pos: usize, by: i64, size: usize) -> usize {
    let newpos = (pos as i64) + by;
    let i_size = size as i64;
    return (((newpos % i_size) + i_size) % i_size) as usize;    
}

impl Shuffle {
    fn new(base: &Vec<i64>, key: i64, times: usize) -> Shuffle {
        let size = base.len();
        return Shuffle { base: base.iter().map(|v| *v * key).collect(),
                         position: (0..size).collect(), size, times }
    }

    fn move_one(&mut self, pos: usize, by: i64) {
        let newpos = modop(pos, by, self.size-1);
        let value = self.position.remove(pos);
        self.position.insert(newpos, value);
    }

    fn shuffle(&mut self) {
        for _ in 0 .. self.times {
            for i in 0..self.size {
                let pos = self.position.iter().position(|c| *c == i).unwrap();
                let by = self.base[i];
                self.move_one(pos, by);
            }
        }
    }

    fn result(&self) -> i64 {
        let shuffled: Vec<i64> = self.position.iter().map(|c| self.base[*c]).collect();
        let zero_pos = shuffled.iter().position(|c| *c == 0).unwrap();
        return
            shuffled[modop(zero_pos, 1000, self.size)] +
            shuffled[modop(zero_pos, 2000, self.size)] +
            shuffled[modop(zero_pos, 3000, self.size)];
 
    }
}

fn read_input(filename: &str) -> Vec<i64> {
    return super::utils::read_lines(filename).iter().map(|l| l.parse::<i64>().unwrap()).collect();
}


pub fn star1(filename: &str) {
    let mut seq = Shuffle::new(&read_input(filename), 1, 1);
    seq.shuffle();
    println!("Star 1: {:?}", seq.result());
}

pub fn star2(filename: &str) {
    let mut seq = Shuffle::new(&read_input(filename), 811589153, 10);
    seq.shuffle();
    println!("Star 2: {:?}", seq.result());
}
