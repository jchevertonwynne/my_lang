#[macro_use]
extern crate lazy_static;
use std::fs;

use crate::lib::{DataStore, Program};

mod lib;

fn main() {
    let f = fs::read_to_string("test.jcw").unwrap();
    let program = f.parse::<Program>().unwrap();

    let mut data_store = DataStore::new();
    program.run(&mut data_store);
}
