use std::fs;

use crate::lib::{DataStore, Program};

mod lib;

fn main() {
    let f = fs::read_to_string("test.jcw").unwrap();

    let lines= f.lines();
    let lines: Vec<&str> = lines.map(str::trim).collect();
    let program = Program::from_lines(&mut lines.iter());

    let mut data_store = DataStore::new();
    program.run(&mut data_store);
}
