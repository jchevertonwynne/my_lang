use regex::Regex;
use core::slice::Iter;

use crate::lib::{DataStore, Expression, Construct};
use std::str::FromStr;

#[derive(Debug)]
pub enum Line {
    Assignment(String, Expression),
    Expression(Expression),
    Construct(Construct)
}

#[derive(Debug)]
pub struct Program {
    pub lines: Vec<Line>
}

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.lines().map(str::trim).map(String::from).collect();
        let mut lines: Iter<String> = lines.iter();
        Ok(Program::from_lines(&mut lines))
    }
}

impl Program {
    pub fn from_lines(lines: &mut Iter<String>) -> Program {
        lazy_static! {
            static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"^([a-z]+): (.+)$").unwrap();
        }

        let mut program = vec![];

        while let Some(line) = lines.next() {
            if line.len() == 0 {
                continue;
            }

            if line == "}" {
                break;
            }

            if let Some(captures) = ASSIGNMENT_REGEX.captures(line) {
                let var = captures[1].to_string();
                let args = captures[2].to_string();
                let exp = Expression::parse(&args).unwrap();
                program.push(Line::Assignment(var, exp));
            } else if let Some(construct) = Construct::parse(&line.to_string(), lines) {
                program.push( Line::Construct(construct));
            } else if let Some(expression) = Expression::parse(&line.to_string()) {
                program.push(Line::Expression(expression));
            }
        }

        Program {
            lines: program
        }
    }

    pub fn run(&self, data_store: &mut DataStore) {
        data_store.expand();
        for line in self.lines.iter() {
            match &line {
                Line::Assignment(var, exp) => {
                    let val = exp.evaluate(data_store).unwrap();
                    data_store.put(var.to_string(), val);
                },
                Line::Expression(exp) => {
                    exp.evaluate(data_store);
                },
                Line::Construct(cons) => {
                    cons.apply(data_store);
                },
            }
        }
        data_store.contract();
    }
}

