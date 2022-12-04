use std::{
    collections::{HashMap, HashSet},
    env, fs,
};
#[macro_use]
extern crate lazy_static;

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

lazy_static! {
    static ref SCORES: HashMap<u8, u32> =
        HashMap::from_iter((b'a'..=b'z').chain(b'A'..=b'Z').zip(1..));
}

fn priority(l: &str) -> Option<&u32> {
    let s: HashSet<u8> = HashSet::from_iter(l[..l.len() / 2].bytes());
    (*SCORES).get(
        &l[l.len() / 2..]
            .bytes()
            .filter(|x| s.contains(x))
            .next()
            .unwrap(),
    )
}

fn part1(inp: &String) {
    println!("{}", inp.lines().map(|l| priority(l).unwrap()).sum::<u32>());
}

fn unique(items: &str) -> HashSet<u8> {
    HashSet::from_iter(items.bytes())
}

fn part2(input: &String) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut score = 0;
    for group in lines.chunks(3) {
        let mut freq: HashMap<u8, usize> = HashMap::new();
        for key in group.iter().map(|g| unique(g)).flatten() {
            if *freq.entry(key).and_modify(|x| *x += 1).or_insert(1) == 3 {
                score += SCORES.get(&key).unwrap();
                break;
            }
        }
    }
    println!("{}", score);
}
