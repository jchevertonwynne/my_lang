use std::fs;

use crate::lib::Program;

mod lib;

fn main() {
    let program_text = fs::read_to_string("test.jcw").unwrap();
    let program_lines = program_text.lines();
    let program_lines: Vec<&str> = program_lines
        .map(str::trim)
        .collect();
    let program = Program::from_lines(&mut program_lines.iter());
    println!("{:?}", program.user_fns);
    program.start();
}
