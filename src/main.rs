use std::fs;

use crate::lib::Program;
use std::collections::HashMap;

mod lib;

fn main() {
    let program_text = fs::read_to_string("programs/test.jcw").unwrap();
    let program_lines = program_text.lines();
    let program_lines: Vec<&str> = program_lines
        .map(str::trim)
        .collect();
    let mut user_fns = HashMap::new();
    let program = Program::from_lines(&mut program_lines.iter(), &mut user_fns);
    let optimised_user_funcs = user_fns.iter().map(|(&k, v)| {
        (k, v.optimise(&user_fns))
    }).collect();
    let program = program.optimise(&optimised_user_funcs);
    program.start();
}
