#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashSet, VecDeque},
    env::var,
    fs::read_to_string,
    iter::repeat,
};

type Point = (i64, i64);
type Block = HashSet<Point>;
type Rows = Block;

fn offset_block(block: &Block, xoff: i64, yoff: i64) -> Block {
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
        .filter(|p| other.contains(p))
        .cloned()
        .next()
        .is_some()
}

fn get_topo(other: &Rows) -> Block {
    if other.len() == 0 {
        return Block::new();
    }

    let yedge = other.iter().map(|p| p.1).max().unwrap() + 1;
    let mut todo = VecDeque::from([(0, yedge)]);
    let mut visited = Block::from([(0, yedge)]);
    let mut ret = Block::new();

    while todo.len() > 0 {
        let curr = todo.pop_front().unwrap();
        if other.contains(&curr) {
            ret.insert(curr);
            continue;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let next = (curr.0 + dx, curr.1 + dy);
                if !visited.contains(&next) && next.1 <= yedge && next.0 >= 0 && next.0 <= 6 {
                    visited.insert(next);
                    todo.push_back(next);
                }
            }
        }
    }
    ret
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
    static ref SKIP : bool = !var("SKIP").is_err();
}

fn main() {
    let mut jet_i = 0;
    let mut block_i = 0;
    let mut other = Rows::from_iter((0..=6).zip(repeat(-1)));
    let mut curr;

    other = get_topo(&other);

    let block_count = i64::from_str_radix(&var("BLOCKS").unwrap(), 10).unwrap();
    let mut t = 0;
    let mut loop_check = vec![];
    let mut keep_check = true;
    let mut bc = 1;

    while bc <= block_count {
        let ymax = *(&other.iter().map(|p| p.1.clone()).max().unwrap());
        curr = offset_block(&BLOCKS[block_i], 2, ymax + 4);
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
                let xmin = curr.iter().cloned().map(|p| p.0).min().unwrap();
                let key = (block_i, xmin, jet_i);

                if keep_check && *SKIP {
                    let needle = loop_check
                        .iter()
                        .enumerate()
                        .filter(|(_, (k, _))| k == &key)
                        .next();

                    if needle.is_some() {
                        println!(
                            "{:?}/{} {} {} {}",
                            needle.unwrap(),
                            loop_check.len(),
                            ymax,
                            block_count,
                            bc
                        );

                        let (idx, (_, start_max)) = needle.unwrap();
                        let dy = ymax - start_max;

                        let before = idx as i64;
                        let remaining = block_count - before;
                        let run = loop_check.len() as i64 - idx as i64;
                        let skip = dy * ((remaining / run) - 1);
                        bc = block_count - (remaining % run);
                        other = offset_block(&other, 0, skip);
                        keep_check = false;

                        println!("Skipping run={} y={} to {}", run, skip, ymax + skip);
                        println!("{} loops, {} left over.", remaining / run, remaining & run);
                        println!("{}", bc);

                        break;
                    } else {
                        loop_check.push((key, ymax)); //ymax before this block
                    }
                }

                other.extend(curr.iter().cloned());
                other = get_topo(&other);

                dump(&Block::new(), &other);
                break;
            }

            dump(&curr, &other);
        }

        bc += 1;
    }

    let ymax = *(&other.iter().map(|p| p.1.clone()).max().unwrap());
    println!("{}", ymax + 1);
}
