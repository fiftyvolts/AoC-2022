#[macro_use]
extern crate lazy_static;
use std::{env::args, fmt, fs::read_to_string, collections::HashSet, iter::once};
use primes::{self, factors};

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
            Operation::Sqr => old * old
        }
    }
    fn exec2(&self, old: Factors) -> Factors {
        match self {
            Operation::Add(x) => Factors::from_iter(primes::factors(old.reduce() + x)),
            Operation::Mul(x) => old.union(&once(*x).collect()).cloned().collect(),
            Operation::Sqr => old
        }
    }

    fn exec3(&self, old: Factors, wrap: u64) -> Factors {
        match self {
            Operation::Add(x) => {
                let mut t = old.iter().cloned().reduce(|a, i| a*i).unwrap();
                t += x;
                t %= wrap;
                primes::factors(t).iter().cloned().collect()
            },
            Operation::Mul(x) => old.union(&HashSet::from(x.factor())).cloned().collect(),
            Operation::Sqr => old
        }
    }

}

#[derive(Debug, Clone, Copy)]
struct Test {
    v: u64,
    yes: usize,
    no: usize,
}

type Factors = HashSet<u64>;
trait Reducible {
    fn reduce(&self) -> u64;
}
impl Reducible for Factors {
    fn reduce(&self) -> u64 {
        self.iter().cloned().reduce(|a, i| a*i).unwrap_or_default()
    }
}

trait Factorable {
    fn factor(&self) -> Factors;
}

impl Factorable for u64 {
    fn factor(&self) -> Factors {
        Factors::from_iter(primes::factors(*self))
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Factors>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Monkey:\n")?;
        writeln!(f, "  Starting items: {:?}", self.items.iter().map(|i| i.reduce()).collect::<Vec<u64>>())?;
        writeln!(f, "  Operation: new = {:?}", self.operation)?;
        writeln!(f, "  Test: divisible by {}", self.test.v)?;
        writeln!(f, "    If true: throw to monkey {}", self.test.yes)?;
        writeln!(f, "    If false: throw to monkey {}", self.test.no)?;
        writeln!(f, "  Inspections: {}", self.inspections)
    }
}
fn main() {
    part1();
    //part2()
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
                .map(|i| u64::from_str_radix(i, 10).unwrap().factor())
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
    println!("{}", monkeys.iter().map(|m| m.to_string()).collect::<Vec<String>>().join("\n"));
    return monkeys;
}

fn part1() {
    let mut monkeys = get_monkeys();
    //let factor = monkeys.iter().map(|m| m.test.v).reduce(|a, v| a*v).unwrap();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();
            monkeys[i].inspections += items.len() as u64;
            for item in items {
                let mut x = monkeys[i].operation.exec2(item);
                x = (x.reduce()/3).factor();
                let j = if x.contains(&monkeys[i].test.v) {
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
    
}

fn print_result(monkeys: Vec<Monkey>) {
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    inspections.sort_by(|a, b| b.cmp(&a));
    println!("{}", inspections[0] * inspections[1]);
    println!("{:?}", inspections);
}
