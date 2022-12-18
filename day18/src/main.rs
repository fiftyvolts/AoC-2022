#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashSet, VecDeque},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref ADJ: Vec<Delta> = vec![
        vec![-1, 0, 0],
        vec![0, -1, 0],
        vec![0, 0, -1],
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ];
}

type Point = Vec<i32>;
type Delta = Vec<i32>;
type Lava = HashSet<Point>;
type Space = HashSet<Point>;

fn unpack(s: &str) -> Point {
    s.split(",")
        .map(|d| i32::from_str_radix(d, 10).unwrap())
        .collect()
}

fn delta(p: &Point, d: &Delta) -> Point {
    vec![p[0] + d[0], p[1] + d[1], p[2] + d[2]]
}

fn get_space(lava: &Lava) -> Space {
    if lava.len() == 0 {
        return Lava::new();
    }

    let first = vec![0,0,0];
    let mut todo = VecDeque::from([first.clone()]);
    let mut visited = Space::from([first.clone()]);
    let mut ret = Space::from([first.clone()]);

    let xmax = lava.iter().max_by(|a, b| a[0].cmp(&b[0])).unwrap()[0];
    let ymax = lava.iter().max_by(|a, b| a[1].cmp(&b[1])).unwrap()[1];
    let zmax = lava.iter().max_by(|a, b| a[2].cmp(&b[2])).unwrap()[2];

    let xrange = 0..=xmax;
    let yrange = 0..=ymax;
    let zrange = 0..=zmax;

    println!("{:?} {:?} {:?}", xmax, ymax, zmax);
    while todo.len() > 0 {
        let curr = todo.pop_front().unwrap();
        if !lava.contains(&curr) {
            ret.insert(curr.clone());
        }

        for d in ADJ.iter() {
            let next = delta(&curr, &d);
            if !visited.contains(&next)
                && xrange.contains(&next[0])
                && yrange.contains(&next[1])
                && zrange.contains(&next[2])
            {
                visited.insert(next.clone());
                todo.push_back(next.clone());
            }
        }
    }
    ret
}

fn main() {
    let lava = Lava::from_iter(INPUT.lines().map(|l| unpack(l)));
    let mut sides = 0;

    for p in &lava {
        for d in ADJ.iter() {
            if !lava.contains(&delta(p, d)) {
                sides += 1;
            }
        }
    }
    println!("{}", sides);

    let space = get_space(&lava);
    sides = 0;
    for p in &space {
        for d in ADJ.iter() {
            if lava.contains(&delta(p, d)) {
                sides += 1;
            }
        }
    }
    println!("{}", sides);
}
