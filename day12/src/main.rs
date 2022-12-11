#[macro_use]
extern crate lazy_static;
use std::{env::args, fs::read_to_string};

lazy_static! {
    static ref INPUT: String = read_to_string(args().nth(1).unwrap()).unwrap();
}

fn main() {
    todo!();
}
