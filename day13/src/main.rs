#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{cmp::Ordering, env::var, fmt::Display, fs::read_to_string, iter::once};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref PAT: Regex = Regex::new(r"\[(?P<list>.*)\]|(?P<int>\d+)").unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
    Null(),
}

impl Packet {
    fn as_list(&self) -> Packet {
        match self {
            Packet::Int(_) => Packet::List(vec![self.clone()]),
            _ => self.clone()
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Packet::Int(x), &Packet::Int(y)) => x.partial_cmp(&y),
            (&Packet::List(ref x), &Packet::List(ref y)) => {
                let mut r = Ordering::Equal;
                for i in 0..(x.len().max(y.len())) {
                    r = x.get(i).cmp(&y.get(i)); //slick comparison of options
                    if r != Ordering::Equal {
                        break;
                    }
                }
                Some(r)
            }
            (&Packet::Int(_), &Packet::List(_)) => self.as_list().partial_cmp(other),
            (&Packet::List(_), &Packet::Int(_)) => self.partial_cmp(&other.as_list()),
            _ => None,
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<&str> for Packet {
    fn from(other: &str) -> Self {
        let caps = PAT.captures(other).unwrap();
        if let Some(next) = caps.name("list") {
            let mut depth = 0;
            let mut last = next.start();
            let mut v = vec![];
            let mut i = next.start();
            while i < next.end() {
                match &other[i..i + 1] {
                    "[" => depth += 1,
                    "]" => depth -= 1,
                    "," => {
                        if depth == 0 {
                            let p = Packet::from(&other[last..i]);
                            v.push(p);
                            last = i + 1;
                        }
                    }
                    _ => (),
                }
                i += 1;
            }
            if depth == 0 && last < i {
                v.push(Packet::from(&other[last..i]));
            }
            Packet::List(v)
        } else if let Some(next) = caps.name("int") {
            Packet::Int(u32::from_str_radix(next.as_str(), 10).unwrap())
        } else {
            Packet::Null()
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(x) => write!(f, "{}", x),
            Packet::List(ref l) => {
                write!(
                    f,
                    "[{}]",
                    l.iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Packet::Null() => write!(f, "NULL"),
        }
    }
}

fn main() {
    let lines = (*INPUT).lines().collect::<Vec<&str>>();
    let s = lines
        .chunks(3)
        .enumerate()
        .filter(|(_, c)| Packet::from(c[0]) < Packet::from(c[1]))
        .map(|(n, _)| n + 1)
        .sum::<usize>();
    println!("{}", s);

    let d1 = Packet::from("[[2]]");
    let d2 = Packet::from("[[6]]");
    let mut packets = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(*l))
        .chain(once(d1.clone()))
        .chain(once(d2.clone()))
        .collect::<Vec<Packet>>();
    packets.sort();

    let (n1, _) = packets.iter().enumerate().find(|(_, p)| **p == d1).unwrap();
    let (n2, _) = packets.iter().enumerate().find(|(_, p)| **p == d2).unwrap();
    println!("{}", (n1 + 1) * (n2 + 1));
}
