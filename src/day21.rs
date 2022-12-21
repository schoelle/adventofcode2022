use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum BinOp { Plus, Minus, Mult, Div, Equal }

impl BinOp {
    fn parse(s: &str) -> BinOp {
        match s {
            "+" => BinOp::Plus,
            "-" => BinOp::Minus,
            "*" => BinOp::Mult,
            "/" => BinOp::Div,
            _ => panic!("Illeagal binary operator")
        }
    }

    fn eval(&self, left: i64, right: i64) -> i64{
        match self {
            BinOp::Plus => left + right,
            BinOp::Minus => left - right,
            BinOp::Mult => left * right,
            BinOp::Div => left / right,
            _ => panic!("Illegal binary operator")
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyFunc {
    Const(i64),
    Human,
    Op(BinOp, Box<MonkeyFunc>, Box<MonkeyFunc>)
}

impl MonkeyFunc {
    fn calc(&self) -> i64 {
        match self {
            MonkeyFunc::Const(v) => *v,
            MonkeyFunc::Op(op, arg1, arg2) => {
                let left = (*arg1).calc();
                let right = (*arg2).calc();
                return op.eval(left, right);
            },
            _ => panic!("Unable to calculate")
        }
    }

    fn simplify(&self) -> MonkeyFunc {
        match self {
            MonkeyFunc::Const(v) => MonkeyFunc::Const(*v),
            MonkeyFunc::Human => MonkeyFunc::Human,
            MonkeyFunc::Op(op, arg1, arg2) => {
                let left = arg1.simplify();
                let right = arg2.simplify();
                return match (&left, &right) {
                    (MonkeyFunc::Const(v1), MonkeyFunc::Const(v2)) => {
                        return MonkeyFunc::Const(op.eval(*v1, *v2));
                    }
                    _ => MonkeyFunc::Op(*op, Box::new(left), Box::new(right))
                }
            }
        }
    }

    fn solve(&self, value: i64) -> i64 {
        match self {
            MonkeyFunc::Const(_) => panic!("Unable to find X"),
            MonkeyFunc::Human => value,
            MonkeyFunc::Op(op, arg1, arg2) => {
                let left = *arg1.clone();
                let right = *arg2.clone();
                return match (left, right) {
                    (v, MonkeyFunc::Const(x)) => match op {
                        BinOp::Plus => v.solve(value - x),
                        BinOp::Minus => v.solve(value + x),
                        BinOp::Mult => v.solve(value / x),
                        BinOp::Div => v.solve(value * x),
                        BinOp::Equal => v.solve(x)
                    },
                    (MonkeyFunc::Const(x), v) => match op {
                        BinOp::Plus => v.solve(value - x),
                        BinOp::Minus => v.solve(x - value),
                        BinOp::Mult => v.solve(value / x),
                        BinOp::Div => v.solve(x / value),
                        BinOp::Equal => v.solve(x)
                    },
                    _ => panic!("Unable to solve")
                }
            }
        }
    }
}

fn build_function(root: &str, caps: &HashMap<String, Vec<String>>, part2: bool) -> MonkeyFunc {
    let cap = &caps[root];
    if cap.len() == 3 {
        if part2 && root == "humn" {
            return MonkeyFunc::Human
        } else {
            return MonkeyFunc::Const(cap[2].parse::<i64>().unwrap());
        }
    } else {
        let left = Box::new(build_function(&cap[3], caps, part2));
        let right = Box::new(build_function(&cap[5], caps, part2));
        if part2 && root == "root" {
            return MonkeyFunc::Op(BinOp::Equal, left, right);
        }
        let op = BinOp::parse(&cap[4]);
        return MonkeyFunc::Op(op, left, right);
    }
}

fn read_function(filename: &str, part2: bool) -> MonkeyFunc {
    let re = Regex::new(r"^(.*): (\d+|(.*) ([+\-*/]) (.*))$").unwrap();
    let mut caps: HashMap<String, Vec<String>> = HashMap::new();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            let name = cap[1].to_string();
            let matches = cap.iter().
                filter_map(|c| c.map(|s| s.as_str().to_string())).collect();
            caps.insert(name, matches);
        }
    }
    return build_function("root", &caps, part2);
}

pub fn star1(filename: &str) {
    let monkeys = read_function(filename, false);
    println!("Star 1: {}", monkeys.calc());
}

pub fn star2(filename: &str) {
    let monkeys = read_function(filename, true).simplify();
    println!("Star 2: {}", monkeys.solve(0));
}
