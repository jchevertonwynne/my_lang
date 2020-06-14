use std::fs;

use crate::lib::Program;
use std::collections::HashMap;

mod lib;

fn main() {
    let program_text = fs::read_to_string("test.jcw").unwrap();
    let program_lines = program_text.lines();
    let program_lines: Vec<&str> = program_lines
        .map(str::trim)
        .collect();
    let mut user_fns = HashMap::new();
    let program = Program::from_lines(&mut program_lines.iter(), &mut user_fns);
    program.start(&user_fns);
}
