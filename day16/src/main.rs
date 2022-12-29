#[macro_use]
extern crate lazy_static;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    env::var,
    fs::read_to_string,
    hash::Hash,
    ops::Index,
    time::SystemTime
};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex =
        Regex::new(r"Valve (\S\S) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    static ref ELEPHANT: bool = var("ELEPHANT").is_ok();
    static ref START_TIME : SystemTime = SystemTime::now();
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
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
    let mut cave: HashMap<&str, Box<Valve>> = HashMap::new();
    for line in INPUT.lines() {
        let valve = Box::new(Valve::from(line));
        cave.insert(valve.name, valve);
    }

    let mut travel: HashMap<(&str, &str), i32> = HashMap::new();
    let valve_names = cave.keys().collect::<Vec<_>>();
    for i in 0..valve_names.len() {
        for j in 0..valve_names.len() {
            let start = valve_names[i];
            let end = valve_names[j];
            if i == j {
                travel.insert((start, start), 0);
                continue;
            }

            let mut que = VecDeque::from([(0, start)]);
            let mut visited = HashSet::<&str>::new();
            while !que.is_empty() {
                let (dist, curr) = que.pop_front().unwrap();
                if curr == end {
                    travel.insert((start, curr), dist);
                    break;
                }
                for adj in &cave.get(curr).unwrap().adj {
                    if !visited.contains(adj) {
                        visited.insert(adj);
                        que.push_back((dist + 1, adj));
                    }
                }
            }
        }
    }

    all_possible_paths(&cave, &travel);
}

fn all_possible_paths(cave: &HashMap<&str, Box<Valve>>, travel: &HashMap<(&str, &str), i32>) {
    let valve_names = cave
        .keys()
        .filter(|k| **k != "AA" && cave[*k].rate > 0)
        .cloned()
        .collect::<BTreeSet<_>>();

    println!(
        "Max solo {}",
        dfs_solo(cave, travel, "AA", valve_names.clone(), 30, 0)
    );

    println!(
        "Max pair {}",
        search_pair(cave, travel, "AA", valve_names, 26)
    );
}

fn search_pair<'a>(
    cave: &HashMap<&str, Box<Valve>>,
    travel: &HashMap<(&str, &str), i32>,
    curr_node: &str,
    remaining: BTreeSet<&'a str>,
    time: i32
) -> i32 {
    let mut max_released = 0;

    let valve_order = Vec::from_iter(remaining);

    for order in PermutationIter::new(valve_order.len()) {
        let remaining1 = order[..order.len() / 2]
            .iter()
            .map(|i| valve_order[*i].clone())
            .collect::<BTreeSet<_>>();

        let mut next_released = dfs_solo(cave, travel, curr_node, remaining1, time, 0);

        let remaining2 = order[order.len() / 2..]
            .iter()
            .map(|i| valve_order[*i].clone())
            .collect::<BTreeSet<_>>();
        next_released += dfs_solo(cave, travel, curr_node, remaining2, time, 0);

        if next_released > max_released {
            println!("New max pair {} (.{})", next_released, START_TIME.elapsed().unwrap().as_millis());
            max_released = next_released;
        }
    }

    max_released
}

fn dfs_solo<'a>(
    cave: &HashMap<&str, Box<Valve>>,
    travel: &HashMap<(&str, &str), i32>,
    curr_node: &str,
    remaining: BTreeSet<&'a str>,
    time: i32,
    best_so_far: i32
) -> i32 {
    let mut release_ceiling = cave[curr_node].rate * time;
    for valve in &remaining {
        release_ceiling += (cave[valve].rate * (time - travel[&(curr_node, *valve)] - 1)).max(0);
    }

    if release_ceiling < best_so_far {
        return 0;
    }

    let mut max_released = 0;
    for next_node in &remaining {
        let next_time = time - travel[&(curr_node, *next_node)] - 1;
        if next_time > 0 {
            let mut next_remaining = remaining.clone();
            next_remaining.remove(next_node);

            let next_released = dfs_solo(
                    cave,
                    travel,
                    next_node,
                    next_remaining,
                    next_time,
                    max_released
                );

            if next_released > max_released {
                max_released = next_released;
            }
        }
    }

    let ret = max_released + cave[curr_node].rate * time;
    ret
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct NamedBitSet<'a, T>
// where
//     T: Index<&'a str, Output = u64>,
// {
//     set: u64,
//     names: &'a T,
// }

// impl<'a, T> NamedBitSet<'a, T>
// where
//     T: Index<&'a str, Output = u64>,
// {
//     fn new(names: &'a T) -> Self {
//         Self { set: 0, names }
//     }

//     fn insert(&mut self, item: &'a str) {
//         self.set |= self.names[item];
//     }

//     fn remove(&mut self, item: &'a str) {
//         self.set &= !(self.names[item]);
//     }

//     fn contains(&self, item: &'a str) -> bool {
//         self.set & self.names[item] > 0
//     }
// }

// impl<'a, T> Hash for NamedBitSet<'a, T>
// where
//     T: Index<&'a str, Output = u64>,
// {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.set.hash(state);
//     }
// }
