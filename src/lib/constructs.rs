use regex::Regex;
use core::slice::Iter;

use crate::lib::{DataStore, Expression, Program};

#[derive(Debug)]
pub enum Construct<'a> {
    If(Expression<'a>, Program<'a>),
    While(Expression<'a>, Program<'a>),
    For(&'a str, Expression<'a>, Expression<'a>, Program<'a>),
}

impl <'a> Construct<'a> {
    pub fn parse(construct: &'a str, lines: &mut Iter<&'a str>) -> Option<Construct<'a>> {
        let if_regex = Regex::new(r"^if (.+) \{$").unwrap();
        let while_regex = Regex::new(r"^while (.+) \{$").unwrap();
        let for_regex = Regex::new(r"^for ([a-z]+) (.*) \{$").unwrap();

        if let Some(capture) = if_regex.captures(&construct) {
            let expression = capture.get(1).unwrap().as_str();
            let expression = Expression::parse(expression).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::If(expression, sub))
        } else if let Some(capture) = while_regex.captures(&construct) {
            let expression = capture.get(1).unwrap().as_str();
            let expression = Expression::parse(expression).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::While(expression, sub))
        } else if let Some(capture) = for_regex.captures(&construct) {
            let iterating = capture.get(1).unwrap().as_str();
            let args = capture.get(2).unwrap().as_str();
            let mut args = Expression::evaluate_arguments(args);
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

    pub fn apply(&self, data_store: &mut DataStore<'a>) {
        match self {
            Construct::If(expr, sub) => {
                if Expression::evaluate(expr, data_store).unwrap() != 0 {
                    sub.run(data_store);
                }
            }
            Construct::While(expr, sub) => {
                while Expression::evaluate(expr, data_store).unwrap() != 0 {
                    sub.run(data_store);
                }
            }
            Construct::For(var, start, end, sub) => {
                data_store.expand();
                let start = Expression::evaluate(start, data_store).unwrap();
                let end = Expression::evaluate(end, data_store).unwrap();
                for i in start..end {
                    data_store.put(*var, i);
                    sub.run(data_store);
                }
                data_store.contract();
            }
        }
    }
}

fn get_sub_program<'a>(lines: &mut Iter<&'a str>) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = vec![];
    let mut brackets = 1;

    while let Some(line) = lines.next() {
        if line.ends_with(&"{") {
            brackets += 1;
        } else if line.eq(&"}") {
            brackets -= 1;
            if brackets == 0 {
                return res;
            }
        }

        res.push(line);
    }

    panic!("unclosed pair of squiggly brackets");
}