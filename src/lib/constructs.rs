use core::slice::Iter;

use regex::Regex;

use crate::lib::{DataStore, Expression, Program, get_sub_program};
use crate::lib::user_function::UserFunction;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Construct<'a> {
    If(Expression<'a>, Program<'a>),
    While(Expression<'a>, Program<'a>),
    For(&'a str, Expression<'a>, Expression<'a>, Program<'a>),
}

impl<'a> Construct<'a> {
    pub fn parse(construct: &'a str, lines: &mut Iter<&'a str>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<Construct<'a>> {
        let if_regex = Regex::new(r"^if (.+) \{$").unwrap();
        let while_regex = Regex::new(r"^while (.+) \{$").unwrap();
        let for_regex = Regex::new(r"^for ([a-z]+) (.*) \{$").unwrap();

        if let Some(capture) = if_regex.captures(&construct) {
            let expression = capture.get(1).unwrap().as_str();
            let expression = Expression::parse(expression, user_fns).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::If(expression, sub))
        } else if let Some(capture) = while_regex.captures(&construct) {
            let expression = capture.get(1).unwrap().as_str();
            let expression = Expression::parse(expression, user_fns).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::While(expression, sub))
        } else if let Some(capture) = for_regex.captures(&construct) {
            let iterating = capture.get(1).unwrap().as_str();
            let args = capture.get(2).unwrap().as_str();
            let mut args = Expression::evaluate_arguments(args, user_fns);
            match args.len() {
                2 => {
                    let start = args.remove(0);
                    let end = args.remove(0);
                    let sub_lines = get_sub_program(lines);
                    let subprogram = Program::from_lines(&mut sub_lines.iter());
                    Some(Construct::For(iterating, start, end, subprogram))
                }
                _ => panic!("invalid for loop \"{}\"", construct),
            }
        } else {
            None
        }
    }

    pub fn apply(&self, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) {
        match self {
            Construct::If(expr, sub) => {
                if Expression::evaluate(expr, data_store, user_fns).unwrap() != 0 {
                    sub.run_with(data_store);
                }
            }
            Construct::While(expr, sub) => {
                while Expression::evaluate(expr, data_store, user_fns).unwrap() != 0 {
                    sub.run_with(data_store);
                }
            }
            Construct::For(var, start, end, sub) => {
                data_store.expand();
                let start = Expression::evaluate(start, data_store, user_fns).unwrap();
                let end = Expression::evaluate(end, data_store, user_fns).unwrap();
                for i in start..end {
                    data_store.put(*var, i);
                    sub.run_with(data_store);
                }
                data_store.contract();
            }
        }
    }
}

