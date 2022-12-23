#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"\d+|[RL]").unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Move(i32),
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => Dir::Move(i32::from_str_radix(&s, 10).unwrap()),
        }
    }
}

type Point = (i32, i32);
#[derive(Debug, Clone, Copy)]
struct Cretin {
    pos: Point,
    face: Facing,
}

struct Tiles {
    tiles: HashMap<Point, Tile>,
    max: Point,
}

impl Tiles {
    fn new(mut tiles: HashMap<Point, Tile>) -> Self {
        let max = (
            *tiles.iter().map(|((x, _), _)| x).max().unwrap() as i32,
            *tiles.iter().map(|((_, y), _)| y).max().unwrap() as i32,
        );
        for x in 0..=max.0 {
            for y in 0..=max.1 {
                if !tiles.contains_key(&(x, y)) {
                    tiles.insert((x, y), Tile::Null);
                }
            }
        }
        Tiles { tiles: tiles, max }
    }

    fn inc_x(&self, p1: Point) -> Point {
        let mut p2 = (p1.0 + 1, p1.1);
        loop {
            if p2.0 > self.max.0 {
                p2.0 = 0;
            }
            match self.tiles[&p2] {
                Tile::Wall => return p1,
                Tile::Open => return p2,
                Tile::Null => p2.0 += 1,
            }
        }
    }

    fn dec_x(&self, p1: Point) -> Point {
        let mut p2 = (p1.0 - 1, p1.1);
        loop {
            if p2.0 < 0 {
                p2.0 = self.max.0;
            }
            match self.tiles[&p2] {
                Tile::Wall => return p1,
                Tile::Open => return p2,
                Tile::Null => p2.0 -= 1,
            }
        }
    }

    fn inc_y(&self, p1: Point) -> Point {
        let mut p2 = (p1.0, p1.1 + 1);
        loop {
            if p2.1 > self.max.1 {
                p2.1 = 0;
            }
            match self.tiles[&p2] {
                Tile::Wall => return p1,
                Tile::Open => return p2,
                Tile::Null => p2.1 += 1,
            }
        }
    }

    fn dec_y(&self, p1: Point) -> Point {
        let mut p2 = (p1.0, p1.1 - 1);
        loop {
            if p2.1 < 0 {
                p2.1 = self.max.1;
            }
            match self.tiles[&p2] {
                Tile::Wall => return p1,
                Tile::Open => return p2,
                Tile::Null => p2.1 -= 1,
            }
        }
    }
}

impl Cretin {
    fn apply(&self, dir: Dir, map: &Tiles) -> Cretin {
        let mut ret = *self;
        match (dir, self.face) {
            (Dir::Move(ds), Facing::Up) => {
                for _ in 0..ds {
                    ret.pos = map.dec_y(ret.pos);
                }
            }
            (Dir::Move(ds), Facing::Down) => {
                for _ in 0..ds {
                    ret.pos = map.inc_y(ret.pos);
                }
            }
            (Dir::Move(ds), Facing::Left) => {
                for _ in 0..ds {
                    ret.pos = map.dec_x(ret.pos);
                }
            }
            (Dir::Move(ds), Facing::Right) => {
                for _ in 0..ds {
                    ret.pos = map.inc_x(ret.pos);
                }
            }

            (Dir::Left, Facing::Up) => ret.face = Facing::Left,
            (Dir::Left, Facing::Down) => ret.face = Facing::Right,
            (Dir::Left, Facing::Left) => ret.face = Facing::Down,
            (Dir::Left, Facing::Right) => ret.face = Facing::Up,

            (Dir::Right, Facing::Up) => ret.face = Facing::Right,
            (Dir::Right, Facing::Down) => ret.face = Facing::Left,
            (Dir::Right, Facing::Left) => ret.face = Facing::Up,
            (Dir::Right, Facing::Right) => ret.face = Facing::Down,
        };
        ret
    }
}

fn main() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let mut tiles = HashMap::new();
    let mut dirs = vec![];
    let mut y = 0;
    while lines[y] != "" {
        for x in 0..lines[y].len() {
            let s = lines[y].get(x..=x).unwrap();
            let p: Point = (x as i32, y as i32);
            match s {
                " " => tiles.insert(p, Tile::Null),
                "." => tiles.insert(p, Tile::Open),
                "#" => tiles.insert(p, Tile::Wall),
                _ => panic!("Bad input"),
            };
        }
        y += 1
    }

    y += 1;
    for dir in RE.find_iter(lines[y]) {
        dirs.push(Dir::from(dir.as_str()));
    }

    let map = Tiles::new(tiles);
    let mut cretin = Cretin {
        pos: (0, 0),
        face: Facing::Right,
    };
    for x in 0..=map.max.0 {
        if map.tiles[&(x, 0)] == Tile::Open {
            cretin.pos = (x, 0);
            break;
        }
    }

    if *DEBUG {
        dump(&map, &cretin);
    }

    for dir in dirs {
        cretin = cretin.apply(dir, &map);
        if *DEBUG {
            println!("{:?}", dir);
            dump(&map, &cretin);
        }
    }
    println!(
        "{}x1000 + {}x4 + {} = {}",
        cretin.pos.1 + 1,
        cretin.pos.0,
        cretin.face as i32,
        (cretin.pos.1 + 1) * 1000 + (cretin.pos.0 + 1) * 4 + cretin.face as i32
    );
}

fn dump(map: &Tiles, cretin: &Cretin) {
    for y in 0..=map.max.1 {
        for x in 0..map.max.0 {
            let p = (x, y);
            if p == cretin.pos {
                match cretin.face {
                    Facing::Up => print!("🔼"),
                    Facing::Down => print!("🔽"),
                    Facing::Left => print!("⏪"),
                    Facing::Right => print!("⏩"),
                }
            } else {
                match map.tiles[&p] {
                    Tile::Open => print!("⬜"),
                    Tile::Wall => print!("🟥"),
                    Tile::Null => print!("⬛"),
                }
            }
        }
        println!("");
    }
    println!("");
}
