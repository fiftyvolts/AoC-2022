#[macro_use]
extern crate lazy_static;
use std::{env::args, fs::read_to_string, ops::RangeInclusive, error::Error};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(args().nth(1).unwrap()).unwrap();
    static ref PAT: Regex = Regex::new(r"(\d+),(\d+)(?: -> )?").unwrap();
}

type Point = (i32, i32);

#[derive(Debug,Clone)]
struct Surface {
    horz: RangeInclusive<i32>,
    vert: RangeInclusive<i32>
}
type Rock = Vec<Surface>;

trait Physics {
    fn collide(&self, rocks: &Vec<Rock>) -> Option<Point>;
    fn slide(&self, rocks: &Vec<Rock>) -> Option<Point>;
}

// impl Physics for Point {
//     fn collide(&self, rocks: &Vec<Rock>) -> Option<Point> {
//         let mut contact = *self;
//         for rock in rocks {
//             for i in 1..rock.len() {
//                 if (rock[i].0..=rock[i - 1].0).contains(&self.0)
//                     && rock[i].1.max(rock[i - 1].1) > contact.1
//                 {
//                     contact = (self.0, rock[i].1.max(rock[i - 1].1));
//                 }
//             }
//         }
//         if contact != *self {
//             return Some(contact);
//         }
//         None
//     }

//     fn slide(&self, rocks: &Vec<Rock>) -> Option<Point> {
//         let mut contact = *self;
//         for rock in rocks {
//             for i in 1..rock.len() {
//                 if (contact.1..contact.1+1).contains(&rock[i].1.max(rock[i - 1].1)) &&
//                 {
//                     // (rock[i].0..=rock[i-1].0).contains(&self.0) &&
//                 }
//             }
//         }

//         None
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let mut rocks = vec![];
    for line in INPUT.lines() {
        let mut rock = Rock::new();
        let mut points = vec![];
        for p in PAT.captures_iter(line){
            points.push((i32::from_str_radix(p.get(1).unwrap().as_str(), 10)?,
                i32::from_str_radix(p.get(2).unwrap().as_str(), 10)?));
            
        }
        for i in 1..points.len() {
            rock.push(Surface {
                horz: (points[i-1].0..=points[i].0),
                vert: (points[i-1].1..=points[i].1)
            });
        }
        rocks.push(rock);
    }

    for rock in rocks {
        println!("{:?}", rock);
    }

    // let mut step = 0;
    // loop {
    //     let mut sand = (500, 0);
    //     step += 1;
    // }
    Ok(())
}
