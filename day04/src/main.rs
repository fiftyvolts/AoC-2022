use std::{env, fs};

fn input_txt() -> String {
    let path = env::args().nth(1).unwrap_or(String::from("ex1.txt"));
    fs::read_to_string(path).ok().unwrap_or_default()
}

fn main() {
    let input = input_txt();
    part1(&input);
    part2(&input);
}

fn parse(i: &str) -> u32 {
    u32::from_str_radix(i, 10).unwrap()
}

fn part1(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let points: Vec<u32> = line.split(&[',', '-']).map(parse).collect();
        let (r1, r2) = (points[0]..=points[1], points[2]..=points[3]);
        if (r1.contains(r2.start()) && r1.contains(r2.end()))
            || (r2.contains(r1.start()) && r2.contains(r1.end()))
        {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let points: Vec<u32> = line.split(&[',', '-']).map(parse).collect();
        let (r1, r2) = (points[0]..=points[1], points[2]..=points[3]);
        if r1.contains(r2.start())
            || r1.contains(r2.end())
            || r2.contains(r1.start())
            || r2.contains(r1.end())
        {
            count += 1;
        }
    }

    println!("{}", count);
}
