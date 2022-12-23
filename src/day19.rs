use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_bot_ore_cost: u64,
    cla_bot_ore_cost: u64,
    obs_bot_ore_cost: u64,
    obs_bot_cla_cost: u64,
    geo_bot_ore_cost: u64,
    geo_bot_obs_cost: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    time_left: u64,
    ore: u64,
    cla: u64,
    obs: u64,
    geo: u64,
    ore_bot: u64,
    cla_bot: u64,
    obs_bot: u64,
    geo_bot: u64
}

impl State {
    fn new(time_left: u64) -> State {
        State { time_left: time_left, ore: 0, cla: 0, obs: 0, geo: 0,
                ore_bot: 1, cla_bot: 0, obs_bot: 0, geo_bot: 0 }
    }
}

impl Blueprint {

    fn geodes(&self, s: State) -> u64 {
        if s.time_left == 0 {
            return s.geo;
        }
        let ore = s.ore + s.ore_bot;
        let cla = s.cla + s.cla_bot;
        let obs = s.obs + s.obs_bot;
        let geo = s.geo + s.geo_bot;
        let time_left = s.time_left - 1;
        let mut best = 0;
        if s.ore >= self.ore_bot_ore_cost && s.ore <= self.ore_bot_ore_cost + s.ore_bot {
            best = max(best, self.geodes(State {
                ore: ore - self.ore_bot_ore_cost,
                ore_bot: s.ore_bot + 1,
                cla, obs, geo, time_left, ..s
            }));
        }
        if s.ore >= self.cla_bot_ore_cost && s.ore <= self.cla_bot_ore_cost + s.ore_bot {
            best = max(best, self.geodes(State {
                ore: ore - self.cla_bot_ore_cost,
                cla_bot: s.cla_bot + 1,
                cla, obs, geo, time_left, ..s
            }));
        }
        if
            s.ore >= self.obs_bot_ore_cost &&
            s.cla >= self.obs_bot_cla_cost &&
            (s.ore < self.obs_bot_ore_cost + s.ore_bot ||
             s.cla < self.obs_bot_cla_cost + s.cla_bot)
        {
            best = max(best, self.geodes(State {
                ore: ore - self.obs_bot_ore_cost,
                cla: cla - self.obs_bot_cla_cost,
                obs_bot: s.obs_bot + 1,
                obs, geo, time_left, ..s
            }));
        }
        if
            s.ore >= self.geo_bot_ore_cost &&
            s.obs >= self.geo_bot_obs_cost &&
            (s.ore < self.geo_bot_ore_cost + s.ore_bot ||
             s.obs < self.geo_bot_obs_cost + s.obs_bot)
        {
            best = max(best, self.geodes(State {
                ore: ore - self.geo_bot_ore_cost,
                obs: obs - self.geo_bot_obs_cost,
                geo_bot: s.geo_bot + 1,
                cla, geo, time_left, ..s
            }));
        }
        best = max(best, self.geodes(State {
            ore, cla, obs, geo, time_left, ..s
        }));
        return best;
    }
}

fn read_blueprints(filename: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let mut res = Vec::new();
    for line in super::utils::read_lines(filename) {
        if let Some(cap) = re.captures(&line) {
            res.push(Blueprint {
                id: cap[1].parse::<u64>().unwrap(),
                ore_bot_ore_cost: cap[2].parse::<u64>().unwrap(),
                cla_bot_ore_cost: cap[3].parse::<u64>().unwrap(),
                obs_bot_ore_cost: cap[4].parse::<u64>().unwrap(),
                obs_bot_cla_cost: cap[5].parse::<u64>().unwrap(),
                geo_bot_ore_cost: cap[6].parse::<u64>().unwrap(),
                geo_bot_obs_cost: cap[7].parse::<u64>().unwrap(),
            });
        } else {
            println!("??? {}", line);
        }
    }
    return res;
}

pub fn star1(filename: &str) {
    let blueprints = read_blueprints(filename);
    let total: u64 = blueprints.iter().map(|bp| {
        let quality = bp.geodes(State::new(24));
        return quality * bp.id;
    }).sum();
    println!("Star 1: {:?}", total);
}

pub fn star2(filename: &str) {
    let blueprints = read_blueprints(filename);
    let total: u64 = blueprints[0..3].iter().map(|bp| {
        let quality = bp.geodes(State::new(32));
        return quality;
    }).product();
    println!("Star 2: {:?}", total);
}
