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

fn part1(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let pair : Vec<Vec<u32>> = line.split(",").map(|p| p.split("-").map(|i| u32::from_str_radix(i, 10).unwrap()).collect::<Vec<u32>>()).collect();
        if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) ||
        (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1]) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let pair : Vec<Vec<u32>> = line.split(",").map(|p| p.split("-").map(|i| u32::from_str_radix(i, 10).unwrap()).collect::<Vec<u32>>()).collect();
        if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1]) ||
        (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][1]) ||

        (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][0]) ||
        (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][0]) ||

        (pair[0][0] <= pair[1][1] && pair[0][1] >= pair[1][1]) ||
        (pair[1][0] <= pair[0][1] && pair[1][1] >= pair[0][1])
        {
            count += 1;
        }
    }

    println!("{}", count);
}