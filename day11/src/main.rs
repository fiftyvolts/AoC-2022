#[macro_use]
extern crate lazy_static;
use std::{env::args, fmt, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(args().nth(1).unwrap()).unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

impl Operation {
    fn exec(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Mul(x) => old * x,
            Operation::Sqr => old * old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    v: u64,
    yes: usize,
    no: usize,
}
#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Monkey:\n")?;
        writeln!(f, "  Starting items: {:?}", self.items)?;
        writeln!(f, "  Operation: new = {:?}", self.operation)?;
        writeln!(f, "  Test: divisible by {}", self.test.v)?;
        writeln!(f, "    If true: throw to monkey {}", self.test.yes)?;
        writeln!(f, "    If false: throw to monkey {}", self.test.no)?;
        writeln!(f, "  Inspections: {}", self.inspections)
    }
}
fn main() {
    part1();
    part2();
}

fn get_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    for chunk in INPUT.lines().collect::<Vec<&str>>().chunks(7) {
        let op_tokens = &chunk[2][23..].split_whitespace().collect::<Vec<&str>>();
        let op = if *op_tokens == vec!["*", "old"] {
            Operation::Sqr
        } else if op_tokens[0] == "+" {
            Operation::Add(u64::from_str_radix(op_tokens[1], 10).unwrap())
        } else if op_tokens[0] == "*" {
            Operation::Mul(u64::from_str_radix(op_tokens[1], 10).unwrap())
        } else {
            panic!("Unknown operations {:?}", op_tokens);
        };

        monkeys.push(Monkey {
            items: (&chunk[1][18..])
                .split(", ")
                .map(|i| u64::from_str_radix(i, 10).unwrap())
                .collect(),
            operation: op,
            test: Test {
                v: u64::from_str_radix(&chunk[3][21..], 10).unwrap(),
                yes: usize::from_str_radix(&chunk[4][29..], 10).unwrap(),
                no: usize::from_str_radix(&chunk[5][30..], 10).unwrap(),
            },
            inspections: 0,
        });
    }
    return monkeys;
}

fn part1() {
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            monkeys[i].inspections += items.len() as u64;

            for item in items {
                let x = monkeys[i].operation.exec(item) / 3;
                let j = if x % monkeys[i].test.v == 0 {
                    monkeys[i].test.yes
                } else {
                    monkeys[i].test.no
                };
                monkeys[j].items.push(x);
            }
        }
    }
    print_result(monkeys);
}

fn part2() {
    let mut monkeys = get_monkeys();
    let factor = monkeys
        .iter()
        .map(|m| m.test.v)
        .reduce(|a, v| a * v)
        .unwrap();
    let scale = |x: u64| -> u64 {
        if x > factor {
            (x % factor) + factor
        } else {
            x
        }
    };

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            monkeys[i].inspections += items.len() as u64;

            for item in items {
                let mut x = monkeys[i].operation.exec(item);
                x = scale(x);
                let j = if x % monkeys[i].test.v == 0 {
                    monkeys[i].test.yes
                } else {
                    monkeys[i].test.no
                };
                monkeys[j].items.push(x);
            }
        }
    }
    print_result(monkeys);
}

fn print_result(monkeys: Vec<Monkey>) {
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.cmp(&a));
    println!("{}", inspections[0] * inspections[1]);
    println!("{:?}", inspections);
}
