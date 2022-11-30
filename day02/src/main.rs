use std::{env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    println!("{}", input);
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    todo!();
}

fn part2(input: &String) {
    todo!();
}