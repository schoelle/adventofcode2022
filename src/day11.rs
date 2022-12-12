use regex::Regex;

type UpdateOp = Box<dyn Fn(i64) -> i64>;

fn build_op(op: &str, arg: &str) -> UpdateOp {
    if arg == "old" {
        if op == "+" {
            return Box::new(|x| x * 2);
        } else {
            return Box::new(|x| x * x);
        }        
    } else {
        let arg_value = arg.parse::<i64>().unwrap();
        if op == "+" {
            return Box::new(move |x| x + arg_value);
        } else {
            return Box::new(move |x| x * arg_value);
        }
    }
}

struct Monkey {
    items: Vec<i64>,
    operation: UpdateOp,
    test_value: i64,
    true_monkey: usize,
    false_monkey: usize,
    inspect_count: u64
}

fn parse_input(filename: &str) -> Vec<Monkey>{
    let monkey_re = Regex::new(r"^Monkey \d+:$").unwrap();
    let items_re = Regex::new(r"^  Starting items: (.*)$").unwrap();
    let ops_re = Regex::new(r"^  Operation: new = old ([+*]) (.*)$").unwrap();
    let test_re = Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    let true_re = Regex::new(r"^    If true: throw to monkey (\d+)$").unwrap();
    let false_re = Regex::new(r"    If false: throw to monkey (\d+)$").unwrap();
    let mut line_iter = super::utils::read_lines(&filename).into_iter();
    let mut result: Vec<Monkey> = Vec::new();
    while let Some(line) = line_iter.next() {
        if monkey_re.is_match(&line) {
            let items_line = line_iter.next().unwrap();
            let items_cap = items_re.captures(&items_line).unwrap();
            let ops_line = line_iter.next().unwrap();
            let ops_cap = ops_re.captures(&ops_line).unwrap();
            let test_line = line_iter.next().unwrap();
            let test_cap = test_re.captures(&test_line).unwrap();
            let true_line = line_iter.next().unwrap();
            let true_cap = true_re.captures(&true_line).unwrap();
            let false_line = line_iter.next().unwrap();
            let false_cap = false_re.captures(&false_line).unwrap();
            let monkey = Monkey {
                items: items_cap[1].split(", ").map(|x| x.parse::<i64>().unwrap()).collect(),
                operation: build_op(&ops_cap[1], &ops_cap[2]),
                test_value: test_cap[1].parse::<i64>().unwrap(),
                true_monkey: true_cap[1].parse::<usize>().unwrap(),
                false_monkey: false_cap[1].parse::<usize>().unwrap(),
                inspect_count: 0
            };
            result.push(monkey);
        }
    }
    return result;
}

fn throw_objects(mut monkeys: Vec<Monkey>, divider: i64, moduler: i64, count: u64) -> u64 {
    for _r in 0..count {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].inspect_count += 1;
                let updated = ((monkeys[i].operation)(item) / divider) % moduler;
                let to = if updated % monkeys[i].test_value == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[to].items.push(updated);
            }
        }
    }
    let mut inspects = monkeys.into_iter().map(|x| x.inspect_count).collect::<Vec<u64>>();
    inspects.sort();
    inspects.reverse();
    return inspects[0] * inspects[1];
}

pub fn star1(filename: &str) {
    let monkeys = parse_input(filename);
    println!("Star 1: {}", throw_objects(monkeys, 3, i64::MAX, 20));
}

pub fn star2(filename: &str) {
    let monkeys = parse_input(filename);
    let moduler = monkeys.iter().map(|x| x.test_value).fold(1, |x,y| x*y);
    println!("Star 2: {}", throw_objects(monkeys, 1, moduler, 10000));
}
