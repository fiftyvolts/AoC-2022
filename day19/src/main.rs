#[macro_use]
extern crate lazy_static;
use std::{env::var, fs::read_to_string};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE : Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

#[derive(Debug,Clone)]
struct Cost {
    ore: i32,
    clay: i32,
    glass: i32,
}

#[derive(Debug,Clone)]
enum Robot {
    Ore(Cost),
    Clay(Cost),
    Glass(Cost),
    Crystal(Cost),
}

#[derive(Debug,Clone)]
struct Blueprint {
    id: i32,
    costs: Vec<Robot>,
    relative_value: Vec<i32>,
    produced: i32,
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

        Blueprint {
            id: caps[0],
            costs: vec![
                Robot::Ore(Cost {
                    ore: caps[1],
                    clay: 0,
                    glass: 0,
                }),
                Robot::Clay(Cost {
                    ore: caps[2],
                    clay: 0,
                    glass: 0,
                }),
                Robot::Glass(Cost {
                    ore: caps[3],
                    clay: caps[4],
                    glass: 0,
                }),
                Robot::Crystal(Cost {
                    ore: caps[5],
                    clay: 0,
                    glass: caps[6],
                }),
            ],
            relative_value: vec![],
            produced: 0,
        }
    }
}

fn main() {
    let mut blueprints = vec![];
    for line in INPUT.lines() {
        blueprints.push(Blueprint::from(line));
    }

    println!("{:?}", blueprints);
}
