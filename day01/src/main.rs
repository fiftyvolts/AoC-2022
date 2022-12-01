use std::{env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part12(&input);
}

fn part12(input: &String) {
    let mut elves: Vec<Vec<u32>> = vec![];
    let mut cur: Vec<u32> = vec![];
    for line in input.lines() {
        match u32::from_str_radix(line, 10) {
            Ok(x) => cur.push(x),
            _ => {
                elves.push(cur);
                cur = vec![];
            }
        }
    }
    let max = elves.iter().map(|elf| elf.iter().sum::<u32>()).max();
    println!("{}", max.unwrap());

    let mut sums = elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<u32>>();

    sums.sort();
    println!("{}", sums.iter().rev().take(3).sum::<u32>());
}
