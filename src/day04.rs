use regex::Regex;

struct Assignment {
    from: u32,
    to: u32
}

impl Assignment {
    fn surrounds(&self, other: &Assignment) -> bool {
        return self.from <= other.from && self.to >= other.to;
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        return (self.to >= other.from) && (self.from <= other.to);
    }
}

pub fn star1(filename: &str) {
    let mut total = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            let first = Assignment { from: cap[1].parse::<u32>().unwrap(),
                                     to: cap[2].parse::<u32>().unwrap()};
            let second = Assignment { from: cap[3].parse::<u32>().unwrap(),
                                      to: cap[4].parse::<u32>().unwrap()};
            if first.surrounds(&second) || second.surrounds(&first) {
                total += 1;
            }
        }
    }
    println!("Star 1: {}", total);
}

pub fn star2(filename: &str) {
    let mut total = 0;
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            let first = Assignment { from: cap[1].parse::<u32>().unwrap(),
                                     to: cap[2].parse::<u32>().unwrap()};
            let second = Assignment { from: cap[3].parse::<u32>().unwrap(),
                                      to: cap[4].parse::<u32>().unwrap()};
            if first.overlaps(&second) {
                total += 1;
            }
        }
    }
    println!("Star 2: {}", total);
}
