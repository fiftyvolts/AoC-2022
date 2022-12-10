#[macro_use]
extern crate lazy_static;
use std::{env, fs};

lazy_static! {
    static ref INPUT: String = env::args()
        .nth(1)
        .and_then(|p| fs::read_to_string(p).ok())
        .unwrap();
}

fn main() {
    part1();
    //part2();
}

fn part1() {
    todo!();
}

// fn part2() {
//     todo!();
// }
