#[macro_use]
extern crate lazy_static;
use std::{collections::HashSet, env::var, fs::read_to_string};

type Point = (i32, i32);
type Block = HashSet<Point>;
type Rows = Block;

fn offset_block(block: &Block, xoff: i32, yoff: i32) -> Block {
    block.iter().map(|(x, y)| (x + xoff, y + yoff)).collect()
}

fn check_wall(block: &Block, others: &Rows) -> bool {
    block
        .iter()
        .filter(|p| p.0 < 0 || p.0 > 6 || others.contains(&p))
        .next()
        .is_some()
}

fn check_below(block: &Block, other: &Rows) -> bool {
    block
        .iter()
        .filter(|p| p.1 < 0 || other.contains(p))
        .cloned()
        .next()
        .is_some()
}

fn dump(block: &Block, other: &Rows) {
    if !*DO_DUMP {
        return;
    }

    let ymax = other.iter().chain(block.iter()).map(|p| p.1).max().unwrap();

    for y in (0..=ymax).rev() {
        print!("|");
        for x in 0..=6 {
            if block.contains(&(x, y)) {
                print!("@");
            } else if other.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref BLOCKS: Vec<Block> = vec![
        Block::from([(0,0), (1,0), (2, 0), (3,0)]), // -
        Block::from([(1,0), (0,1), (1,1), (2,1), (1,2)]), // +
        Block::from([(0,0), (1,0), (2,0), (2,1), (2,2)]), // L backwards
        Block::from([(0,0), (0,1), (0,2), (0,3)]), // |
        Block::from([(0,0), (1,0), (0,1), (1,1)]) // []
    ];
    static ref DO_DUMP : bool = !var("DUMP").is_err();
}

fn main() {
    let mut jet_i = 0;
    let mut block_i = 0;
    let mut other = Rows::new();
    let mut ymax = 0;
    let mut curr;

    let block_count = usize::from_str_radix(&var("BLOCKS").unwrap(), 10).unwrap();
    let mut t = 0;
    for _ in 0..block_count {
        curr = offset_block(&BLOCKS[block_i], 2, 3 + ymax);
        block_i = (block_i + 1) % BLOCKS.len();
        dump(&curr, &other);
        loop {
            if *DO_DUMP {
                println!("{} {} {}", t, &INPUT[jet_i..=jet_i], jet_i);
            }
            t += 1;

            let xoff = match &INPUT[jet_i..=jet_i] {
                "<" => -1,
                ">" => 1,
                _ => panic!("Bad input"),
            };
            jet_i = (jet_i + 1) % INPUT.len();

            let next = offset_block(&curr, xoff, 0);
            if !check_wall(&next, &other) {
                curr = next;
            }

            let next = offset_block(&curr, 0, -1);

            if !check_below(&next, &other) {
                curr = next;
            } else {
                let curr_ymax = *(&curr.iter().map(|p| p.1.clone()).max().unwrap());
                other.extend(curr.iter().cloned());
                if curr_ymax + 1 > ymax {
                    ymax = curr_ymax + 1
                }
                dump(&curr, &other);
                break;
            }

            dump(&curr, &other);
        }
    }

    println!("{}", ymax);
}
