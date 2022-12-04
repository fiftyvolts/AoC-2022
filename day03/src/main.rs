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
    static ref SCORES: HashMap<u8, u32> = HashMap::from_iter(
        (0..26)
            .map(|i| [(b'a' + i, 1 + i as u32), (b'A' + i, 27 + i as u32)])
            .flatten()
    );
}

fn priority(l: &str) -> Option<&u32> {
    let s: HashSet<u8> = HashSet::from_iter(l.bytes().take(l.len() / 2));
    (*SCORES).get(
        &l.bytes()
            .skip(l.len() / 2)
            .filter(|x| s.contains(x))
            .next()
            .unwrap(),
    )
}

fn part1(input: &String) {
    println!(
        "{}",
        input.lines().map(|l| priority(l).unwrap()).sum::<u32>()
    );
}

fn part2(input: &String) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut score = 0;
    for i in (0..lines.len()).step_by(3) {
        let mut item_count: HashMap<u8, usize> = HashMap::new();
        for j in 0..3 {
            for key in HashSet::<u8>::from_iter(lines[i + j].bytes()) {
                let count = item_count.entry(key).and_modify(|x| *x += 1).or_insert(1);
                if *count == 3 {
                    score += SCORES.get(&key).unwrap();
                    break;
                }
            }
        }
    }
    println!("{}", score);
}
