use regex::Regex;
use core::slice::Iter;

use crate::lib::{DataStore, Expression, Construct};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Line<'a> {
    Assignment(&'a str, Expression<'a>),
    Expression(Expression<'a>),
    Construct(Construct<'a>)
}

#[derive(Debug)]
pub struct Program<'a> {
    pub lines: Vec<Line<'a>>,
    pub user_fns: HashMap<String, Program<'a>>
}

impl <'a> Program<'a> {
    pub fn from_lines(lines: &mut Iter<&'a str>) -> Program<'a> {
        let assignment_regex = Regex::new(r"^([a-z]+): (.+)$").unwrap();

        let mut program = vec![];

        while let Some(&line) = lines.next() {
            if line.len() == 0 {
                continue;
            }

            if line == "}" {
                break;
            }

            if let Some(captures) = assignment_regex.captures(line) {
                let var = captures.get(1).unwrap().as_str();
                let args = captures.get(2).unwrap().as_str();
                let exp = Expression::parse(args).unwrap();
                program.push(Line::Assignment(var, exp));
            } else if let Some(construct) = Construct::parse(&line, lines) {
                program.push( Line::Construct(construct));
            } else if let Some(expression) = Expression::parse(line) {
                program.push(Line::Expression(expression));
            }
        }

        Program {
            lines: program,
            user_fns: HashMap::new()
        }
    }

    pub fn run(&self, data_store: &mut DataStore<'a>) {
        data_store.expand();
        for line in self.lines.iter() {
            match &line {
                Line::Assignment(var, exp) => {
                    let val = exp.evaluate(data_store).unwrap();
                    data_store.put(var, val);
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

