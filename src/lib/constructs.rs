use regex::Regex;
use core::slice::Iter;

use crate::lib::{DataStore, Expression, Program};

#[derive(Debug)]
pub enum Construct {
    If(Expression, Program),
    While(Expression, Program),
    For(String, Expression, Expression, Program),
}

impl Construct {
    pub fn parse(construct: &String, lines: &mut Iter<String>) -> Option<Construct> {
        lazy_static! {
            static ref IF_REGEX: Regex = Regex::new(r"^if (.+) \{$").unwrap();
            static ref WHILE_REGEX: Regex = Regex::new(r"^while (.+) \{$").unwrap();
            static ref FOR_REGEX: Regex = Regex::new(r"^for ([a-z]+) (.*) \{$").unwrap();
        }

        if let Some(capture) = IF_REGEX.captures(&construct) {
            let expression = Expression::parse(&capture[1].to_string()).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::If(expression, sub))
        } else if let Some(capture) = WHILE_REGEX.captures(&construct) {
            let expression = Expression::parse(&capture[1].to_string()).unwrap();
            let sub_lines = get_sub_program(lines);
            let sub = Program::from_lines(&mut sub_lines.iter());
            Some(Construct::While(expression, sub))
        } else if let Some(capture) = FOR_REGEX.captures(&construct) {
            let iterating = capture[1].to_string();
            let mut args = Expression::evaluate_arguments(&capture[2].to_string());
            match args.len() {
                2 => {
                    let start = args.remove(0);
                    let end = args.remove(0);
                    let sub_lines = get_sub_program(lines);
                    let sub = Program::from_lines(&mut sub_lines.iter());
                    Some(Construct::For(iterating, start, end, sub))
                }
                _ => panic!("invalid for loop \"{}\"", construct),
            }
        } else {
            None
        }
    }

    pub fn apply(&self, data_store: &mut DataStore) {
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
                    data_store.put(var.to_string(), i);
                    sub.run(data_store);
                }
                data_store.contract();
            }
        }
    }
}

fn get_sub_program(lines: &mut Iter<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut brackets = 1;

    while let Some(line) = lines.next() {
        if line.ends_with(&String::from("{")) {
            brackets += 1;
        } else if line.eq(&String::from("}")) {
            brackets -= 1;
            if brackets == 0 {
                return res;
            }
        }

        res.push(line.to_string());
    }

    panic!("unclosed pair of squiggly brackets");
}