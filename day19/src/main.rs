#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE : Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    static ref SCALE : i32 = i32::from_str_radix(var("SCALE").unwrap_or(String::from("100")).as_str(), 10).unwrap();
}

#[derive(Debug, Clone)]
struct Cost {
    ore: i32,
    clay: i32,
    glass: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Robot {
    Ore,
    Clay,
    Glass,
    Crystal,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    costs: HashMap<Robot, Cost>
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        let caps = RE
            .captures(input)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| i32::from_str_radix(m.unwrap().as_str(), 10).unwrap())
            .collect::<Vec<_>>();

        let ore_bot = Cost {
            ore: caps[1],
            clay: 0,
            glass: 0,
        };

        let clay_bot = Cost {
            ore: caps[2],
            clay: 0,
            glass: 0,
        };

        let glass_bot = Cost {
            ore: caps[3],
            clay: caps[4],
            glass: 0,
        };

        let crystal_bot = Cost {
            ore: caps[5],
            clay: 0,
            glass: caps[6],
        };
        Blueprint {
            id: caps[0],
            costs: HashMap::from([
                (Robot::Ore, ore_bot),
                (Robot::Clay, clay_bot),
                (Robot::Glass, glass_bot),
                (Robot::Crystal, crystal_bot),
            ]),
        }
    }
}

fn calculate_output(bp: &Blueprint) -> i32 {
0
}
fn main() {
    let blueprints = INPUT.lines().map(|l| Blueprint::from(l)).collect::<Vec<_>>();
    let mut results = vec![];
    for bp in &blueprints {
        results.push((bp.id, calculate_output(bp)));
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));

    println!("{:?}", results);
}
