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

    // for x in 0..=max {
    //     'points: for y in 0..=max {
    //         if y == 0 && x % 10 == 0{
    //             print!(".\n");
    //         }
    //         for b in &beacons {
    //             // println!("{},{} {} {}",x,y, b.ds, (x - b.point.0).abs() + (y - b.point.1).abs());
    //             if b.ds >= (x - b.point.0).abs() + (y - b.point.1).abs() {
    //                 continue 'points;
    //             }
    //         }
    //         println!("{},{} {}", x, y, x * 4000000 + y)
    //     }
    // }

    for i in 0..beacons.len() {
        let b0 = &beacons[i];
        let ymin = if b0.point.1 - b0.ds - 1 < 0 {
            0
        } else {
            b0.point.1 - b0.ds
        };
        let ymax = if b0.point.1 + b0.ds + 1 > max {
            max
        } else {
            b0.point.1 + b0.ds
        };
        let mut check = vec![];
        for y in ymin..ymax {
            let dx = b0.ds + 1 - (y - b0.point.1).abs();
            if b0.point.0 + dx > 0 && b0.point.0 + dx < max {
                check.push((b0.point.0 + dx, y));
            }
            if b0.point.0 - dx > 0 && b0.point.0 - dx < max {
                check.push((b0.point.0 - dx, y));
            }
        }
        let mut found = HashSet::new();
        for j in 0..beacons.len() {
            if i == j {
                continue;
            }
            let b1 = &beacons[j];
            for c in &check {
                let ds_c = (c.0 - b1.point.0).abs() + (c.1 - b1.point.1).abs();
                if ds_c <= b1.ds {
                    found.insert(*c);
                }
            }
        }
        let orig = HashSet::from_iter(check);
        let dif = orig.difference(&found).collect::<Vec<_>>();
        if dif.len() > 0 {
            let x = dif[0].0;
            let y = dif[0].1;
            println!("{},{} = {}", x, y, x as u64 * 4000000 as u64 + y as u64);
            return Ok(());
        }
    }

    // for y in 0..max {
    //     // let mut covered = [0xffu8; 3];
    //     // covered[2] &= 0x0f;
    //     if y%1000 == 0 {
    //     println!(".");
    //     }
    //     let mut covered = [0xffu8;500001];
    //     covered[500000] &= 0x01;
    //     for b in &beacons {
    //         let dtarget = b.ds - (y - b.point.1).abs();
    //         if dtarget > 0 {
    //             let lower = if b.point.0 - dtarget < 0 {
    //                 0
    //             } else {
    //                 b.point.0 - dtarget
    //             };
    //             let upper = if b.point.0 + dtarget > max {
    //                 max
    //             } else {
    //                 b.point.0 + dtarget
    //             };
    //             for i in lower..=upper {
    //                 covered[(i / 8) as usize] &= !(1 << i % 8);
    //             }
    //         }
    //     }
    //     for (i, d) in covered.iter().enumerate() {
    //         if *d != 0 {
    //             let x = i as u32 * 8 + d.trailing_zeros();
    //             println!("{},{} = {}", x, y, x as u64 * 4000000 as u64 + y as u64);
    //             return Ok(());
    //         }
    //     }
    // }

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
