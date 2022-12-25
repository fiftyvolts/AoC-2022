#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"\d+|[RL]").unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
    static ref PART2: bool = var("PART2").is_ok();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Null,
    Warp(fn(Cretin) -> Cretin, char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

        Tiles { tiles, max }
    }

    fn inc_x(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1;
        c2.pos.0 += 1;
        loop {
            if c2.pos.0 > self.max.0 {
                c2.pos.0 = 0;
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => return c1,
                Tile::Open => return c2,
                Tile::Null => c2.pos.0 += 1,
                Tile::Warp(warp, c) =>  {
                    if *DEBUG {
                        println!("Warp {} from {:?} to {:?}", c, c1, c2);
                    }
                    return warp(c2)
                },
            }
        }
    }

    fn dec_x(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1;
        c2.pos.0 -= 1;
        loop {
            if c2.pos.0 < 0 {
                c2.pos.0 = self.max.0;
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => return c1,
                Tile::Open => return c2,
                Tile::Null => c2.pos.0 -= 1,
                Tile::Warp(warp, c) =>  {
                    if *DEBUG {
                        println!("Warp {} from {:?} to {:?}", c, c1, c2);
                    }
                    return warp(c2)
                },
            }
        }
    }

    fn inc_y(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1;
        c2.pos.1 += 1;
        loop {
            if c2.pos.1 > self.max.1 {
                c2.pos.1 = 0;
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => return c1,
                Tile::Open => return c2,
                Tile::Null => c2.pos.1 += 1,
                Tile::Warp(warp, c) =>  {
                    if *DEBUG {
                        println!("Warp {} from {:?} to {:?}", c, c1, c2);
                    }
                    return warp(c2)
                },
            }
        }
    }

    fn dec_y(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1;
        c2.pos.1 -= 1;
        loop {
            if c2.pos.1 < 0 {
                c2.pos.1 = self.max.1;
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => return c1,
                Tile::Open => return c2,
                Tile::Null => c2.pos.1 -= 1,
                Tile::Warp(warp, c) =>  {
                    if *DEBUG {
                        println!("Warp {} from {:?} to {:?}", c, c1, c2);
                    }
                    return warp(c2)
                },
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
                    ret = map.dec_y(ret);
                }
            }
            (Dir::Move(ds), Facing::Down) => {
                for _ in 0..ds {
                    ret = map.inc_y(ret);
                }
            }
            (Dir::Move(ds), Facing::Left) => {
                for _ in 0..ds {
                    ret = map.dec_x(ret);
                }
            }
            (Dir::Move(ds), Facing::Right) => {
                for _ in 0..ds {
                    ret = map.inc_x(ret);
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
            let p : Point = if *PART2 {
                (x as i32 , y as i32 )
            } else {
                (x as i32, y as i32)
            };
            
            match s {
                " " => tiles.insert(p, Tile::Null),
                "." => tiles.insert(p, Tile::Open),
                "#" => tiles.insert(p, Tile::Wall),
                _ => tiles.insert(p, WARPS[s]),
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
    for y in -2..=map.max.1 + 1 {
        for x in -2..map.max.0 + 1 {
            let p = (x, y);

            if p == cretin.pos {
                match cretin.face {
                    Facing::Up => print!("🔼"),
                    Facing::Down => print!("🔽"),
                    Facing::Left => print!("⏪"),
                    Facing::Right => print!("⏩"),
                }
            } else if map.tiles.contains_key(&p) {
                match map.tiles[&p] {
                    Tile::Open => print!("⬜"),
                    Tile::Wall => print!("🟥"),
                    Tile::Null => print!("⬛"),
                    Tile::Warp(_,c) => print!("{}", c),
                }
            } else {
                print!("💢")
            }
        }
        println!("");
    }
    println!("");
}

lazy_static! {
    static ref WARPS: HashMap<String, Tile> = HashMap::from([
        (String::from("1"), Tile::Warp(warp_1, '①')),
        (String::from("2"), Tile::Warp(warp_2, '②')),
        (String::from("3"), Tile::Warp(warp_3, '③')),
        (String::from("4"), Tile::Warp(warp_4, '④')),
        (String::from("5"), Tile::Warp(warp_5, '⑤')),
        (String::from("6"), Tile::Warp(warp_6, '⑥')),
        (String::from("7"), Tile::Warp(warp_7, '⑦')),
        (String::from("8"), Tile::Warp(warp_8, '⑧')),
        (String::from("9"), Tile::Warp(warp_9, '⑨')),
        (String::from("A"), Tile::Warp(warp_a, '🅰')),
        (String::from("B"), Tile::Warp(warp_b, '🅱')),
        (String::from("C"), Tile::Warp(warp_c, '🅲')),
        (String::from("D"), Tile::Warp(warp_d, '🅳')),
        (String::from("E"), Tile::Warp(warp_e, '🅴')),
        (String::from("F"), Tile::Warp(warp_f, '🅵')),
        (String::from("G"), Tile::Warp(warp_g, '🅶')),
        (String::from("H"), Tile::Warp(warp_h, '🅷'))
    ]);
}
fn warp_1(c0: Cretin) -> Cretin {
    Cretin {
        pos: (c0.pos.1 - 100, 0),
        face: Facing::Down,
    }
}

fn warp_2(c0: Cretin) -> Cretin {
    Cretin {
        pos: (c0.pos.1 - 100, 149),
        face: Facing::Up,
    }
}

fn warp_3(c0: Cretin) -> Cretin {
    Cretin {
        pos: (c0.pos.0 + 100, 0),
        face: Facing::Down,
    }
}

fn warp_4(c0: Cretin) -> Cretin {
    Cretin {
        pos: (49, c0.pos.0 + 100),
        face: Facing::Left,
    }
}

fn warp_5(c0: Cretin) -> Cretin {
    Cretin {
        pos: (149, c0.pos.1 - 100),
        face: Facing::Left,
    }
}

fn warp_6(c0: Cretin) -> Cretin {
    Cretin {
        pos: (c0.pos.1 + 50, 149),
        face: Facing::Up,
    }
}

fn warp_7(c0: Cretin) -> Cretin { //checked
    Cretin {
        pos: (c0.pos.1 - 50, 100),
        face: Facing::Down,
    }
}

fn warp_8(c0: Cretin) -> Cretin {
    Cretin {
        pos: (50, c0.pos.0 + 50),
        face: Facing::Right,
    }
}

fn warp_9(c0: Cretin) -> Cretin {
    Cretin {
        pos: (50, c0.pos.1 - 100),
        face: Facing::Right,
    }
}


fn warp_a(c0: Cretin) -> Cretin {
    Cretin {
        pos: (0, c0.pos.1 + 100),
        face: Facing::Right,
    }
}


fn warp_b(c0: Cretin) -> Cretin {
    Cretin {
        pos: (99, c0.pos.0 - 50),
        face: Facing::Left,
    }
}

fn warp_c(c0: Cretin) -> Cretin {
    Cretin {
        pos: (99, c0.pos.1 + 100),
        face: Facing::Left,
    }
}


fn warp_d(c0: Cretin) -> Cretin {
    Cretin {
        pos: (0, c0.pos.0 + 100),
        face: Facing::Right,
    }
}

fn warp_h(c0: Cretin) -> Cretin {
    Cretin {
        pos: (c0.pos.0 - 100, 149),
        face: Facing::Up,
    }
}

fn warp_e(c0: Cretin) -> Cretin {
    if c0.face == Facing::Down {
        warp_b(c0)
    } else {
        warp_6(c0)
    }
}

fn warp_f(c0: Cretin) -> Cretin {
    if c0.face == Facing::Up {
        warp_8(c0)
    } else {
        warp_7(c0)
    }
}


fn warp_g(c0: Cretin) -> Cretin {
    if c0.face == Facing::Right {
        warp_2(c0)
    } else {
        warp_4(c0)
    }
}