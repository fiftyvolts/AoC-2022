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

    _all_possible_paths(&cave, &travel);
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

fn _all_possible_paths_cached_incremental(
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
    let mut memo = HashMap::<String, (&str, usize, i32, i32)>::new();
    
    for i in 10.min(valve_names.len())..=valve_names.len() {
        println!("{} out of {} deep", i, valve_names.len());
        let mut next_memo = HashMap::new();

        for order in PermutationIter::new(i) {
            let cache_key = order
                .iter()
                .map(|x| valve_names[*x])
                .cloned()
                .collect::<String>();

            let mut curr = "AA";
            let mut time = 30;
            let mut release = 0;
            let mut idx = 0;

            if let Some(hit) = memo.get(&cache_key[..cache_key.len() - 2]) {
                (curr, idx, time, release) = *hit;
            }

            let mut next_idx = 0;
            for j in idx..order.len() {
                next_idx = j;
                let next = valve_names[order[j]].as_str();
                let dt = 1 + travel[&(curr, next)].len() as i32;
                if dt > time {
                    time = 0;
                    break;
                }
                time -= dt;
                release += time * cave[next].rate;
                curr = next;
            }

            next_memo.insert(cache_key, (curr, next_idx + 1, time, release));
            if release > max_release {
                max_release = release;
                max_order = order;
                println!("new max {} {:?}", max_release, max_order);
            }
        }

        memo = next_memo;
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

fn _all_possible_paths(
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
            println!("new max {} {:?}", max_release, max_order);
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

fn _find_next_bext_valve(
    cave: &HashMap<String, Box<Valve>>,
    travel: &HashMap<(&str, &str), Vec<&str>>,
) {
    let valve_names = cave.keys().collect::<Vec<_>>();
    let mut time = 0;
    let mut release = 0;
    let mut curr = "AA";
    let mut opened = HashSet::<&str>::new();
    loop {
        let mut max_release = 0;
        let mut max_name = "";
        let mut max_dt = 0;

        for i in 0..valve_names.len() {
            let next_name = valve_names[i];
            if opened.contains(next_name.as_str()) {
                continue;
            }

            let path = travel.get(&(curr, next_name)).unwrap();
            let next_time = 1 + path.len() as i32 + time;

            if next_time >= 30 {
                continue;
            }

            let next_valve = cave.get(next_name).unwrap();
            let next_release = (30 - next_time) * next_valve.rate;
            if next_release > max_release {
                max_name = next_name;
                max_release = next_release;
                max_dt = next_time;
            }
        }

        if max_release == 0 {
            break;
        }

        opened.insert(max_name);
        release += max_release;
        time = max_dt;
        curr = max_name;
        println!(
            "At {} openeing {} releasing {}",
            time, max_name, max_release
        );
    }

    println!("released {}", release);
}
