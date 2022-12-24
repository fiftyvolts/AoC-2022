#[macro_use]
extern crate lazy_static;
use std::{
    collections::{HashSet, VecDeque},
    env::var,
    fs::read_to_string,
};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref DEBUG: bool = var("DEBUG").is_ok();
}

type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Facing {
    fn from(c: char) -> Self {
        match c {
            '^' => Facing::Up,
            'v' => Facing::Down,
            '<' => Facing::Left,
            '>' => Facing::Right,
            _ => panic!("Unknown facing"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    face: Facing,
    p: Point,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Mount {
    bliz: Vec<Blizzard>,
    start: Point,
    end: Point,
    curr: Point,
    max: Point,
}

impl Mount {
    fn inc_bliz(&self, b: Blizzard) -> Point {
        match b.face {
            Facing::Up => {
                if b.p.1 == 0 {
                    (b.p.0, self.max.1)
                } else {
                    (b.p.0, (b.p.1 - 1))
                }
            }
            Facing::Down => {
                if b.p.1 == self.max.1 {
                    (b.p.0, 0)
                } else {
                    (b.p.0, (b.p.1 + 1))
                }
            }
            Facing::Left => {
                if b.p.0 == 0 {
                    (self.max.0, b.p.1)
                } else {
                    ((b.p.0 - 1), b.p.1)
                }
            }
            Facing::Right => {
                if b.p.0 == self.max.0 {
                    (0, b.p.1)
                } else {
                    (b.p.0 + 1, b.p.1)
                }
            }
        }
    }

    fn step(&self) -> Self {
        Self {
            bliz: self
                .bliz
                .iter()
                .map(|b| Blizzard {
                    p: self.inc_bliz(*b),
                    face: b.face,
                })
                .collect(),
            start: self.start,
            end: self.end,
            curr: self.curr,
            max: self.max,
        }
    }

    fn adj(&self, p0: Point) -> Vec<Point> {
        [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)]
            .iter()
            .filter_map(|ds| {
                let p1 = (p0.0 + ds.0, p0.1 + ds.1);
                if p1 == self.start
                    || p1 == self.end
                    || (p1.0 >= 0 && p1.0 <= self.max.0 && p1.1 >= 0 && p1.1 <= self.max.1)
                {
                    Some(p1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn dump(&self) {
        println!("{:?} / {:?} => {:?}", self.curr, self.max, self.end);
        print!(" 🟦");
        for x in 0..=self.max.0 {
            if x == self.start.0 {
                if self.curr == self.start {
                    print!("🧝");
                } else {
                print!("⬜");
                }
            } else {
                print!("🟦");
            }
        }
        println!("🟦");
        for y in 0..=self.max.1 {
            print!(" 🟦");
            'grid: for x in 0..=self.max.0 {
                for b in &self.bliz {
                    if (x, y) == b.p {
                        match b.face {
                            Facing::Up => print!("⏫"),
                            Facing::Down => print!("⏬"),
                            Facing::Left => print!("⏪"),
                            Facing::Right => print!("⏩"),
                        }
                        continue 'grid;
                    }
                }
                if self.curr == (x, y) {
                    print!("🧝");
                } else {
                    print!("⬜");
                }
            }

            println!("🟦");
        }
        print!(" 🟦");
        for x in 0..=self.max.0 {
            if x == self.end.0 {
                print!("⬜");
            } else {
                print!("🟦");
            }
        }
        println!("🟦");
    }
}

fn load() -> Mount {
    let mut mount: Mount = Mount {
        bliz: vec![],
        start: (-1, -1),
        end: (-1, -1),
        curr: (-1, -1),
        max: (-1, -1),
    };
    let mut last_open = -1;

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => continue,
                '.' => last_open = x as i32 - 1,
                '^' | 'v' | '<' | '>' => mount.bliz.push(Blizzard {
                    face: Facing::from(c),
                    p: (x as i32 - 1, y as i32 - 1),
                }),
                _ => panic!("Unknown character in input {}", c),
            }

            if mount.max.0 < x as i32 - 1 {
                mount.max.0 = x as i32 - 1;
            }

            if mount.max.1 < y as i32 - 2 {
                mount.max.1 = y as i32 - 2;
            }
        }
        if y as i32 - 1 == -1 {
            mount.start = (last_open, -1);
        }
    }
    mount.end = (last_open, mount.max.1 + 1);
    mount.curr = mount.start;
    mount
}

fn main() {
    let mount = load();

    if *DEBUG {
        mount.dump();
    }

    println!("{}", round(&mount));
}

fn round(initial: &Mount) -> u32 {
    let mut queue = VecDeque::from([(0, initial.clone())]);
    let mut visited = HashSet::new();

    while let Some((step,  mount)) = queue.pop_front() {
        visited.insert(mount.clone());

        if *DEBUG {
            mount.dump();
        }

        let opts = mount.adj(mount.curr);
        let mut next = mount.step();
        'check_opt: for opt in opts {
            if opt == mount.end {
                return step + 1;
            }

            for b in &next.bliz {
                if opt == b.p {
                    continue 'check_opt;
                }
            }

            next.curr = opt;
            if !visited.contains(&next) {
                queue.push_back((step + 1,  next.clone()));
            }
        }
    }

    return 0;
}
