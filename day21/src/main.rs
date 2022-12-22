#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashMap, env::var, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT").unwrap()).unwrap();
    static ref RE: Regex = Regex::new(r"([a-z]+): (?:(\d+)|([a-z]+) ([+-/*]) ([a-z]+))").unwrap();
}

#[derive(Debug, Clone)]
enum Exp {
    Literal(String, i64),
    Equation(String, String, String, String),
}

impl Exp {
    fn name(&self) -> &String {
        match self {
            Exp::Literal(name, _) => &name,
            Exp::Equation(name, _, _, _) => &name,
        }
    }

    fn eval(&self, lut: &HashMap<String, Exp>) -> i64 {
        match self {
            Exp::Literal(_, x) => *x,
            Exp::Equation(_, a, op, b) => match op.as_str() {
                "+" => lut[a].eval(lut) + lut[b].eval(lut),
                "-" => lut[a].eval(lut) - lut[b].eval(lut),
                "*" => lut[a].eval(lut) * lut[b].eval(lut),
                "/" => lut[a].eval(lut) / lut[b].eval(lut),
                _ => panic!("No such operator."),
            },
        }
    }
}
fn main() {
    let mut lut = HashMap::new();
    let mut rev = HashMap::new();

    for line in INPUT.lines() {
        let caps = RE.captures(line).unwrap();
        let exp = if caps.get(2).is_some() {
            Exp::Literal(
                caps[1].to_string(),
                i64::from_str_radix(&caps[2], 10).unwrap(),
            )
        } else {
            Exp::Equation(
                caps[1].to_string(),
                caps[3].to_string(),
                caps[4].to_string(),
                caps[5].to_string(),
            )
        };
        lut.insert(exp.name().clone(), exp);
    }

    println!("part 1: {}", lut["root"].eval(&lut));

    
}
