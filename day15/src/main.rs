#[macro_use]
extern crate lazy_static;
use std::{collections::HashSet, env::var, error::Error, fs::read_to_string};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}
#[derive(Debug)]
struct Beacon {
    point: (i32, i32),
    closest: (i32, i32),
    ds: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut beacons = vec![];
    for line in INPUT.lines() {
        let cap = PAT.captures(line).unwrap();
        let point = (
            i32::from_str_radix(cap.get(1).unwrap().as_str(), 10)?,
            i32::from_str_radix(cap.get(2).unwrap().as_str(), 10)?,
        );
        let closest = (
            i32::from_str_radix(cap.get(3).unwrap().as_str(), 10)?,
            i32::from_str_radix(cap.get(4).unwrap().as_str(), 10)?,
        );
        let ds = (closest.0 - point.0).abs() + (closest.1 - point.1).abs();
        beacons.push(Beacon { point, closest, ds });
    }

    let max = i32::from_str_radix(var("MAX").unwrap().as_str(), 10).unwrap();
    
    for x in 0..=max {
        'points: for y in 0..=max {
            if y == 0 && x % 10 == 0{
                print!(".\n");
            }
            for b in &beacons {
                // println!("{},{} {} {}",x,y, b.ds, (x - b.point.0).abs() + (y - b.point.1).abs());
                if b.ds >= (x - b.point.0).abs() + (y - b.point.1).abs() {
                    continue 'points;
                }
            }
            println!("{},{} {}", x, y, x * 4000000 + y)
        }
    }

    // let mut covered = HashSet::new();
    // let mut beacons_set = HashSet::new();

    // let target = 2000000;
    // for (_, b) in beacons.iter().enumerate() {
    //     let dtarget = b.ds - (target - b.point.1).abs();
    //     if dtarget > 0 {
    //         for i in (b.point.0 - dtarget)..=(b.point.0 + dtarget) {
    //             covered.insert(i);
    //         }
    //     }
    //     if b.closest.1 == target {
    //         beacons_set.insert(b.closest.0);
    //     }
    // }
    // let mut sorted = Vec::from_iter(covered);
    // sorted.sort();
    // //println!("{:?}", sorted);
    // println!("{}", sorted.len() - beacons_set.len());
    Ok(())
}
