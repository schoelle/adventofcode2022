use regex::Regex;
use std::result::Result;

fn rock_paper_scissors_value(token: &str) -> Result<i32, &'static str> {
    match token {
        "A"|"X" => Ok(0),
        "B"|"Y" => Ok(1),
        "C"|"Z" => Ok(2),
        _ => Err("Unknown Token")
    }
}

fn rock_paper_scissors_result(opp_token: &str, my_token: &str) -> Result<i32, &'static str> {
    let opp_value = rock_paper_scissors_value(opp_token)?;
    let my_value = rock_paper_scissors_value(my_token)?;
    let game_value = ((my_value - opp_value + 4) % 3) * 3;
    let play_value = my_value + 1;
    return Ok(game_value + play_value);
}

fn choose_my_move(opp_token: &str, instruction: &str) -> Result<&'static str, &'static str> {
    let opp_value = rock_paper_scissors_value(opp_token)?;
    let instr_value = rock_paper_scissors_value(instruction)?;
    return match (opp_value + instr_value) % 3 {
        0 => Ok("Z"),
        1 => Ok("X"),
        2 => Ok("Y"),
        _ => Err("Unable to choose move")
    }
}

pub fn star1(filename: &str) {
    let re = Regex::new(r"^(.) (.)").unwrap();
    let mut total = 0;
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            if let Ok(res) = rock_paper_scissors_result(&cap[1], &cap[2]) {
                total += res;
            }
        }
    }
    println!("Star 1: {}", total);
}

pub fn star2(filename: &str) {
    let re = Regex::new(r"^(.) (.)").unwrap();
    let mut total = 0;
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            if let Ok(my_move) = choose_my_move(&cap[1], &cap[2]) {
                if let Ok(res) = rock_paper_scissors_result(&cap[1], my_move) {
                    total += res;
                }
            }
        }
    }
    println!("Star 1: {}", total);
}
