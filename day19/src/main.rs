#[macro_use]
extern crate lazy_static;
use std::{env::var, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(var("INPUT")).unwrap()).unwrap();
}

fn main() {
    todo!();
}
