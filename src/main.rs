#[macro_use] extern crate lazy_static;
use std::fs;

use crate::lib::{DataStore, interpret};

mod lib;

fn main() {
    let f = fs::read_to_string("test.jcw").unwrap();
    let lines = f.lines();
    let f: Vec<String> = lines
        .map(str::trim)
        .map(String::from)
        .collect();

    let mut data_store = DataStore::new();

    interpret(&f, &mut data_store);
}
