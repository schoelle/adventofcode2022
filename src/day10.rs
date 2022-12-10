use regex::Regex;

#[derive(Debug)]
struct State {
    time: i32,
    regx: i32,
    sum: i32
}

type Callback = fn(&mut State);

fn cb1(state: &mut State) {
    if (state.time - 20) % 40 == 0 {
        state.sum += state.time * state.regx;
    }
}

fn cb2(state: &mut State) {
    let pos = state.time % 40;
    if pos == 0 {
        println!("");
    }
    if (state.regx - pos).abs() <= 1 {
        print!("*");
    } else {
        print!(" ");
    }
}

fn simulate(filename: &str, callback: Callback, start_time: i32) -> State {
    let re = Regex::new(r"^(addx|noop)( (-?\d+)|)$").unwrap();
    let mut state = State { time: start_time, regx: 1, sum: 0 };
    for line in super::utils::read_lines(&filename) {
        if let Some(cap) = re.captures(&line) {
            match &cap[1] {
                "noop" => {
                    callback(&mut state);
                    state.time += 1;
                 }
                "addx" => {
                    callback(&mut state);
                    state.time += 1;
                    callback(&mut state);
                    state.time += 1;
                    state.regx += &cap[3].parse::<i32>().unwrap();
                }
                &_ => panic!("Unknown command")
            }
        }
    }
    return state
}

pub fn star1(filename: &str) {
    println!("Star 1: {}", simulate(filename, cb1, 1).sum);
}

pub fn star2(filename: &str) {
    println!("Star 2:");
    simulate(filename, cb2, 0);
}
