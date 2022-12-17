#[macro_use]
extern crate lazy_static;
use std::{collections::HashSet, env::var, fs::read_to_string};

type Point = (i32, i32);
type Block = Vec<Point>;

fn offset_block(block: &Block, xoff: i32, yoff: i32) -> Block {
    block.iter().map(|(x, y)| (x + xoff, y + yoff)).collect()
}

fn check_wall(block: &Block, others: &HashSet<Point>) -> bool {
    block
        .iter()
        .filter(|(x, y)| x < &0 || x > &6 || others.contains(&(*x, *y)))
        .next()
        .is_some()
}

fn check_below(block: &Block, other: &HashSet<Point>) -> bool {
    block
        .iter()
        .filter(|p| other.contains(p) || p.1 < 0)
        .cloned()
        .next()
        .is_some()
}

fn dump(points: &HashSet<Point>, block: &Vec<Point>) {
    if !*DO_DUMP {
        return;
    }

    let mut block_set = HashSet::new();
    block_set.extend(block.iter().cloned());
    let ymax = points
        .iter()
        .chain(block_set.iter())
        .map(|p| p.1)
        .max()
        .unwrap();
    for y in (0..=ymax).rev() {
        print!("|");
        for x in 0..=6 {
            if block_set.contains(&(x, y)) {
                print!("@");
            } else if points.contains(&(x, y)) {
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
        vec![(0,0), (1,0), (2, 0), (3,0)], // -
        vec![(1,0), (0,1), (1,1), (2,1), (1,2)], // +
        vec![(0,0), (1,0), (2,0), (2,1), (2,2)], // L backwards
        vec![(0,0), (0,1), (0,2), (0,3)], // |
        vec![(0,0), (1,0), (0,1), (1,1)] // []
    ];
    static ref DO_DUMP : bool = !var("DUMP").is_err();
}

fn main() {
    let mut jet_i = 0;
    let mut block_i = 0;
    let mut other = HashSet::new();
    let mut ymax = 0;
    let mut curr;

    let block_count = usize::from_str_radix(&var("BLOCKS").unwrap(), 10).unwrap();
    let mut t = 0;
    for _ in 0..block_count {
        curr = offset_block(&BLOCKS[block_i], 2, 3+ymax);
        block_i = (block_i + 1) % BLOCKS.len();
        dump(&other, &curr);
        loop {
            if *DO_DUMP {
            println!("{} {} {}", t, &INPUT[jet_i..=jet_i], jet_i);
            }
            t+=1;

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
                let next_ymax = *(&curr.iter().map(|p| p.1.clone()).max().unwrap()) + 1;
                if next_ymax > ymax {
                    ymax = next_ymax
                }
                other.extend(curr.iter());
                dump(&other, &vec![]);
                break;
            }



            dump(&other, &curr);
        }
    }

    println!("{}", ymax);
}
