use regex::Regex;
use core::slice::Iter;

use crate::lib::{Line, DataStore, Expression, Construct};

pub struct Program {
    pub lines: Vec<Line>
}

impl Program {
    pub fn parse(lines: &Vec<String>) -> Program {
        lazy_static! {
            static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"^([a-z]+): (.+)$").unwrap();
        }

        let mut program = vec![];
        let mut lines: Iter<String> = lines.iter();

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
                // let val = Expression::evaluate(&exp, data_store).unwrap();
                // data_store.put(var, val);
            } else if let Some(construct) = Construct::parse(&line.to_string(), &mut lines) {
                program.push( Line::Construct(construct));
                // construct.apply(sub_lines, data_store);
            } else if let Some(expression) = Expression::parse(&line.to_string()) {
                program.push(Line::Expression(expression));
                // expression.evaluate(data_store);
            }
        }

        Program{
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

