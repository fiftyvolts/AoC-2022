#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::var,
    fs::read_to_string,
    iter::once,
};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex =
        Regex::new(r"Valve (\S\S) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    static ref ELEPHANT : bool = var("ELEPHANT").is_ok();
    static ref GENETIC : bool = var("GENETIC").is_ok();
    static ref SEED : String = var("SEED").unwrap_or_default();
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: String,
    rate: i32,
    adj: Vec<&'a str>,
}

impl<'a> From<&'a str> for Valve<'a> {
    fn from(input: &'a str) -> Self {
        let caps = PAT.captures(&input).unwrap();
        Valve {
            name: caps.get(1).unwrap().as_str().into(),
            rate: i32::from_str_radix(caps.get(2).unwrap().as_str(), 10).unwrap(),
            adj: caps.get(3).unwrap().as_str().split(", ").collect(),
        }
    }
}

fn main() {
    let mut cave: HashMap<String, Box<Valve>> = HashMap::new();
    for line in INPUT.lines() {
        let valve = Box::new(Valve::from(line));
        cave.insert(valve.name.clone(), valve);
    }

    let mut travel: HashMap<(&str, &str), Vec<&str>> = HashMap::new();
    let valve_names = cave.keys().collect::<Vec<_>>();
    for i in 0..valve_names.len() {
        for j in 0..valve_names.len() {
            let start = valve_names[i].as_str();
            let end = valve_names[j].as_str();
            if i == j {
                travel.insert((start, start), vec![]);
                continue;
            }

            let mut que = VecDeque::from([(vec![], start)]);
            let mut visited = HashSet::<&str>::new();
            while !que.is_empty() {
                let (path, curr) = que.pop_front().unwrap();
                if curr == end {
                    travel.insert((start, curr), path);
                    break;
                }
                for adj in &cave.get(curr).unwrap().adj {
                    if !visited.contains(adj) {
                        visited.insert(adj);
                        que.push_back((
                            path.iter().chain(once(adj)).cloned().collect::<Vec<_>>(),
                            adj,
                        ));
                    }
                }
            }
        }
    }

    if *GENETIC {
        genetic(&cave, &travel);
    } else if *ELEPHANT {
        elephant_paths(&cave, &travel);
    } else {
        all_possible_paths(&cave, &travel);
    }
}

fn genetic(cave: &HashMap<String, Box<Valve>>, travel: &HashMap<(&str, &str), Vec<&str>>) {
    let mut valve_names : Vec<String>;
    
    if *SEED != "" {
        valve_names = SEED.split(",").map(|s| String::from(s)).collect::<Vec<_>>();
    } else {
        valve_names = cave
        .keys()
        .filter(|k| k != &"AA" && cave[*k].rate > 0)
        .cloned()
        .collect::<Vec<_>>();
        valve_names.sort_by(|a, b| cave[a].rate.cmp(&cave[b].rate));
    }

    println!("{:?}", *SEED);
    let path : Vec<&str> = valve_names.iter().map(|s| s.as_str()).collect();
    println!("{}", modal_path_pressure(cave, travel, &path[..]));

    // for i in 0..valve_names.len() {
    //     for j in 0..valve_names.len() {

    //     }
    // }

}

fn modal_path_pressure(
    cave: &HashMap<String, Box<Valve>>,
    travel: &HashMap<(&str, &str), Vec<&str>>,
    path: &[&str]
) -> i32 {
    if *ELEPHANT {
        path_pressure(cave, travel, &path[..path.len()/2], 26) + 
        path_pressure(cave, travel, &path[path.len()/2..], 26)
    } else {
        path_pressure(cave, travel, path, 30)
    }
}

fn path_pressure(
    cave: &HashMap<String, Box<Valve>>,
    travel: &HashMap<(&str, &str), Vec<&str>>,
    path: &[&str],
    start_time: i32
) -> i32 {
    let mut curr = "AA";
    let mut time = start_time;
    let mut release = 0;
    for i in 0..path.len() {
        let next = path[i];
        
        let dt = 1 + travel[&(curr, next)].len() as i32;
        if dt > time {
            return release;
        }
        curr = next;
        time -= dt;
        release += time * cave[next].rate;
        println!("{} {}", next, release);
    }
    release
}


struct PermutationIter {
    regs: Vec<usize>,
    done: bool,
}

impl PermutationIter {
    fn new(size: usize) -> PermutationIter {
        PermutationIter {
            regs: vec![0; size],
            done: false,
        }
    }
}

impl Iterator for PermutationIter {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut indexes: Vec<usize> = (0..self.regs.len()).collect();
        let ret = self.regs.iter().map(|i| indexes.remove(*i)).collect();

        let mut carry = 1;
        for i in 0..self.regs.len() {
            self.regs[i] = self.regs[i] + carry;
            if self.regs[i] >= (self.regs.len() - i) {
                self.regs[i] = 0;
                carry = 1;
            } else {
                carry = 0;
            }
        }

        if carry == 1 {
            self.done = true;
        }

        Some(ret)
    }
}
fn all_possible_paths(
    cave: &HashMap<String, Box<Valve>>,
    travel: &HashMap<(&str, &str), Vec<&str>>,
) {
    let mut valve_names = cave
        .keys()
        .filter(|k| k != &"AA" && cave[*k].rate > 0)
        .collect::<Vec<_>>();
    valve_names.sort();

    let mut max_release = 0;
    let mut max_order = vec![];
    for order in PermutationIter::new(valve_names.len()) {
        let mut curr = "AA";
        let mut time = 30;
        let mut release = 0;

        for i in 0..order.len() {
            let next = valve_names[order[i]].as_str();
            let dt = 1 + travel[&(curr, next)].len() as i32;
            if dt > time {
                break;
            }
            time -= dt;
            release += time * cave[next].rate;
            curr = next;
        }
        if release > max_release {
            max_release = release;
            max_order = order;
            println!(
                "new max {} {:?}",
                max_release,
                max_order
                    .iter()
                    .map(|k| valve_names[*k])
                    .collect::<Vec<_>>()
            );
        }
    }

    println!(
        "{} {:?}",
        max_release,
        max_order
            .iter()
            .map(|k| valve_names[*k])
            .collect::<Vec<_>>()
    );
}

fn elephant_paths(cave: &HashMap<String, Box<Valve>>, travel: &HashMap<(&str, &str), Vec<&str>>) {
    let mut valve_names = cave
        .keys()
        .filter(|k| k != &"AA" && cave[*k].rate > 0)
        .collect::<Vec<_>>();

    valve_names.sort();

    let mut max_release = 0;
    let mut max_order = vec![];
    for order in PermutationIter::new(valve_names.len()) {
        let mut curr = "AA";
        let mut time = 26;
        let mut release = 0;

        //agent 1
        for i in 0..order.len() / 2 {
            let next = valve_names[order[i]].as_str();
            let dt = 1 + travel[&(curr, next)].len() as i32;
            if dt > time {
                break;
            }
            time -= dt;
            release += time * cave[next].rate;
            curr = next;
        }

        //agent 2
        curr = "AA";
        time = 26;
        for i in order.len() / 2..order.len() {
            let next = valve_names[order[i]].as_str();
            let dt = 1 + travel[&(curr, next)].len() as i32;
            if dt > time {
                break;
            }
            time -= dt;
            release += time * cave[next].rate;
            curr = next;
        }

        if release > max_release {
            max_release = release;
            max_order = order;
            println!(
                "new max {} {:?}",
                max_release,
                max_order
                    .iter()
                    .map(|k| valve_names[*k])
                    .collect::<Vec<_>>()
            );
        }
    }

    println!(
        "{} {:?}",
        max_release,
        max_order
            .iter()
            .map(|k| valve_names[*k])
            .collect::<Vec<_>>()
    );
}
