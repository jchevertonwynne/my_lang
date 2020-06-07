#[macro_use]
extern crate lazy_static;
use std::fs;

use crate::lib::{DataStore, Program};

mod lib;

fn main() {
    let f = fs::read_to_string("test.jcw").unwrap();
    let lines = f.lines();
    let f: Vec<String> = lines.map(str::trim).map(String::from).collect();

    let mut data_store = DataStore::new();

    let program = Program::parse(&f);
    program.run(&mut data_store);
}
