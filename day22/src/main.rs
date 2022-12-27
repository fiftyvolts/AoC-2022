#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, env::var, fs::read_to_string, sync::Mutex};

use regex::Regex;

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"\d+|[RL]").unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
    static ref TRACE: bool = var("TRACE").is_ok();
    static ref TRACE_PATH: bool = var("TRACE_PATH").is_ok();
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

    fn wrap_steps(&self, c1: Cretin) -> Cretin {
        let mut c2 = c1.step();
        loop {
            if !*PART2 {
                c2.pos.0 = c2.pos.0.rem_euclid(self.max.0 + 1);
                c2.pos.1 = c2.pos.1.rem_euclid(self.max.1 + 1);
            }
            match self.tiles[&c2.pos] {
                Tile::Wall => {
                    wall_trace(c2);
                    return c1;
                }
                Tile::Open => return c2,
                Tile::Null => {
                    if *PART2 {
                        panic!("Hit a null {:?}", c2);
                    }
                    c2 = c2.step();
                }
                Tile::Warp(warp, c) => {
                    c2 = warp(c2.unstep());
                    warp_trace(c, c1, c2);
                }
            }
        }
    }
}
lazy_static! {
    static ref TRACE_MAP: Mutex<HashMap<Point, Cretin>> = Mutex::new(HashMap::new());
}

fn trace(c: Cretin) {
    if *TRACE_PATH {
        TRACE_MAP.lock().unwrap().insert(c.pos, c);
    }

    if *TRACE {
        println!("{:?}", c);
    }
}

fn warp_trace(c: char, from: Cretin, to: Cretin) {
    if *TRACE_PATH {
        TRACE_MAP.lock().unwrap().insert(to.pos, to);
    }
    if *TRACE {
        println!("Warp {} from {:?} to {:?}", c, from, to);
    }
}

fn wall_trace(cn: Cretin) {
    if *TRACE {
        println!("Hit wall at {:?}", cn);
    }
}

impl Cretin {
    fn step(&self) -> Self {
        let mut cn = *self;
        match cn.face {
            Facing::Right => cn.pos.0 += 1,
            Facing::Down => cn.pos.1 += 1,
            Facing::Left => cn.pos.0 -= 1,
            Facing::Up => cn.pos.1 -= 1,
        }
        cn
    }

    fn unstep(&self) -> Self {
        let mut cn = *self;
        match cn.face {
            Facing::Right => cn.pos.0 -= 1,
            Facing::Down => cn.pos.1 -= 1,
            Facing::Left => cn.pos.0 += 1,
            Facing::Up => cn.pos.1 += 1,
        }
        cn
    }

    fn apply(&self, dir: Dir, map: &Tiles) -> Cretin {
        let mut ret = *self;
        match (dir, self.face) {
            (Dir::Move(ds), Facing::Up)
            | (Dir::Move(ds), Facing::Down)
            | (Dir::Move(ds), Facing::Left)
            | (Dir::Move(ds), Facing::Right) => {
                for _ in 0..ds {
                    ret = map.wrap_steps(ret);
                    trace(ret);
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
            let p: Point = if *PART2 {
                (x as i32 - 1, y as i32 - 1)
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

    trace(cretin);

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
    if *TRACE_PATH {
        dump_trace(&map, &TRACE_MAP.lock().unwrap());
    }

    println!(
        "{}x1000 + {}x4 + {} = {}",
        cretin.pos.1 + 1,
        cretin.pos.0 + 1,
        cretin.face as i32,
        (cretin.pos.1 + 1) * 1000 + (cretin.pos.0 + 1) * 4 + cretin.face as i32
    );
}

fn dump(map: &Tiles, cretin: &Cretin) {
    dump_trace(map, &HashMap::from([(cretin.pos, cretin.clone())]));
}

fn dump_trace(map: &Tiles, cretins: &HashMap<Point, Cretin>) {
    for y in -2..=map.max.1 + 1 {
        for x in -2..map.max.0 + 1 {
            let p = (x, y);

            if cretins.contains_key(&p) {
                let cretin = cretins[&p];
                match cretin.face {
                    Facing::Up => print!("🔼"),
                    Facing::Down => print!("🔽"),
                    Facing::Left => print!("⏪"),
                    Facing::Right => print!("⏩"),
                }
            } else if map.tiles.contains_key(&p) {
                match map.tiles[&p] {
                    Tile::Open => print!("🟫"),
                    Tile::Wall => print!("🟥"),
                    Tile::Null => print!("⬛"),
                    Tile::Warp(_, c) => print!("{}", c),
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
        (String::from("1"), Tile::Warp(warp_1_d, '①')),
        (String::from("2"), Tile::Warp(warp_2_4, '②')),
        (String::from("3"), Tile::Warp(warp_3_h, '③')),
        (String::from("4"), Tile::Warp(warp_4_2, '④')),
        (String::from("5"), Tile::Warp(warp_5_c, '⑤')),
        (String::from("6"), Tile::Warp(warp_6_b, '⑥')),
        (String::from("7"), Tile::Warp(warp_7_8, '⑦')),
        (String::from("8"), Tile::Warp(warp_8_7, '⑧')),
        (String::from("9"), Tile::Warp(warp_9_a, '⑨')),
        (String::from("A"), Tile::Warp(warp_a_9, '🅰')),
        (String::from("B"), Tile::Warp(warp_b_6, '🅱')),
        (String::from("C"), Tile::Warp(warp_c_5, '🅲')),
        (String::from("D"), Tile::Warp(warp_d_1, '🅳')),
        (String::from("E"), Tile::Warp(warp_e, '🅴')),
        (String::from("F"), Tile::Warp(warp_f, '🅵')),
        (String::from("G"), Tile::Warp(warp_g, '🅶')),
        (String::from("H"), Tile::Warp(warp_h_3, '🅷'))
    ]);
}

struct _Warp {
    enter_face: Facing,
    exit_face: Facing,
    enter_start: Point,
    exit_start: Point,
    size: i32,
}

impl _Warp {
    fn _warp(&self, cn: Cretin) -> Cretin {
        assert_eq!(cn.face, self.enter_face);
        assert!(cn.pos.0 >= self.enter_start.0);
        assert!(cn.pos.0 < self.enter_start.0 + self.size);
        assert!(cn.pos.1 >= self.enter_start.1);
        assert!(cn.pos.1 <= self.enter_start.1 + self.size);

        let ds = match self.enter_face {
            Facing::Right | Facing::Left => cn.pos.1 % self.size,
            Facing::Up | Facing::Down => cn.pos.0 % self.size,
        };

        let pos = match self.exit_face {
            Facing::Right | Facing::Left => (self.exit_start.0, self.exit_start.1 + ds),
            Facing::Up | Facing::Down => (self.exit_start.0 + ds, self.exit_start.1),
        };

        let face = self.exit_face;
        Cretin { pos, face }
    }
}

fn warp_1_d(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Left);
    let x = 50 + (cn.pos.1 % 50);
    let y = 0;
    Cretin {
        pos: (x, y),
        face: Facing::Down,
    }
}

fn warp_2_4(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Right);
    let x = 50 + (cn.pos.1 % 50);
    let y = 149;
    Cretin {
        pos: (x, y),
        face: Facing::Up,
    }
}

fn warp_3_h(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Down);
    let x = 100 + (cn.pos.0 % 50);
    let y = 0;
    Cretin {
        pos: (x, y),
        face: Facing::Down,
    }
}

fn warp_4_2(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Down);
    let x = 49;
    let y = 150 + (cn.pos.0 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Left,
    }
}

fn warp_5_c(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Right);
    let x = 149;
    let y = 0 + (cn.pos.1 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Left,
    }
}

fn warp_6_b(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Right);
    let x = 100 + (cn.pos.1 % 50);
    let y = 49;
    Cretin {
        pos: (x, y),
        face: Facing::Up,
    }
}

fn warp_7_8(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Left);
    let x = 0 + (cn.pos.1 % 50);
    let y = 100;
    Cretin {
        pos: (x, y),
        face: Facing::Down,
    }
}

fn warp_8_7(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Up);
    let x = 50;
    let y = 50 + (cn.pos.0 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Right,
    }
}

fn warp_9_a(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Left);
    let x = 50;
    let y = 0 + (cn.pos.1 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Right,
    }
}

fn warp_a_9(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Left);
    let x = 0;
    let y = 100 + (cn.pos.1 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Right,
    }
}

fn warp_b_6(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Down);
    let x = 99;
    let y = 50 + (cn.pos.1 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Left,
    }
}

fn warp_c_5(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Right);
    let x = 99;
    let y = 100 + (cn.pos.1 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Left,
    }
}

fn warp_d_1(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Up);
    let x = 0;
    let y = 150 + (cn.pos.0 % 50);
    Cretin {
        pos: (x, y),
        face: Facing::Right,
    }
}

fn warp_h_3(cn: Cretin) -> Cretin {
    assert!(cn.face == Facing::Up);
    let x = 0 + (cn.pos.0 % 50);
    let y = 199;
    Cretin {
        pos: (x, y),
        face: Facing::Up,
    }
}

fn warp_e(cn: Cretin) -> Cretin {
    if cn.face == Facing::Down {
        warp_b_6(cn)
    } else {
        warp_6_b(cn)
    }
}

fn warp_f(cn: Cretin) -> Cretin {
    if cn.face == Facing::Up {
        warp_8_7(cn)
    } else {
        warp_7_8(cn)
    }
}

fn warp_g(cn: Cretin) -> Cretin {
    if cn.face == Facing::Right {
        warp_2_4(cn)
    } else {
        warp_4_2(cn)
    }
}
