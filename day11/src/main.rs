#[macro_use]
extern crate lazy_static;
use std::{env::args, fmt, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(args().nth(1).unwrap()).unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(u32),
    Mul(u32),
    Sqr,
}

impl Operation {
    fn exec(&self, old: u32) -> u32 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Mul(x) => old * x,
            Operation::Sqr => old * old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Test {
    v: u32,
    yes: usize,
    no: usize,
}
#[derive(Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: Test,
    inspections: u32,
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
    run(3, 20);
}

fn run(worry_div: u32, rounds: usize) {
    let mut monkeys: Vec<Monkey> = vec![];
    for chunk in INPUT.lines().collect::<Vec<&str>>().chunks(7) {
        let op_tokens = &chunk[2][23..].split_whitespace().collect::<Vec<&str>>();
        let op = if *op_tokens == vec!["*", "old"] {
            Operation::Sqr
        } else if op_tokens[0] == "+" {
            Operation::Add(u32::from_str_radix(op_tokens[1], 10).unwrap())
        } else if op_tokens[0] == "*" {
            Operation::Mul(u32::from_str_radix(op_tokens[1], 10).unwrap())
        } else {
            panic!("Unknown operations {:?}", op_tokens);
        };

        monkeys.push(Monkey {
            items: (&chunk[1][18..])
                .split(", ")
                .map(|i| u32::from_str_radix(i, 10).unwrap())
                .collect(),
            operation: op,
            test: Test {
                v: u32::from_str_radix(&chunk[3][21..], 10).unwrap(),
                yes: usize::from_str_radix(&chunk[4][29..], 10).unwrap(),
                no: usize::from_str_radix(&chunk[5][30..], 10).unwrap(),
            },
            inspections: 0,
        });
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            monkeys[i].inspections += items.len() as u32;

            for item in items {
                let x = monkeys[i].operation.exec(item) / worry_div;
                let j = if x % monkeys[i].test.v == 0 {
                    monkeys[i].test.yes
                } else {
                    monkeys[i].test.no
                };
                monkeys[j].items.push(x);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u32>>();
    inspections.sort_by(|a, b| b.cmp(&a));
    println!("{}", inspections[0] * inspections[1]);
    println!("{:?}", inspections);
}
