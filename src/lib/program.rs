use core::slice::Iter;
use std::collections::HashMap;

use regex::Regex;

use crate::lib::{Construct, DataStore, Expression};
use crate::lib::user_function::UserFunction;

#[derive(Debug)]
pub enum Line<'a> {
    Assignment(&'a str, Expression<'a>),
    Expression(Expression<'a>),
    Construct(Construct<'a>),
}

#[derive(Debug)]
pub struct Program<'a> {
    pub program: Vec<Line<'a>>,
    pub user_fns: HashMap<&'a str, UserFunction<'a>>,
}

impl<'a> Program<'a> {
    pub fn from_lines(lines: &mut Iter<&'a str>) -> Program<'a> {
        let assignment_regex = Regex::new(r"^([a-z]+): (.+)$").unwrap();
        let fn_regex = Regex::new(r"^func ([a-z]+) \((.+)\) \{$").unwrap();

        let mut program = vec![];
        let mut user_fns = HashMap::new();

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
                let exp = Expression::parse(args, &user_fns).unwrap();
                program.push(Line::Assignment(var, exp));
            } else if let Some(construct) = Construct::parse(line, lines, &user_fns) {
                program.push(Line::Construct(construct));
            } else if let Some(captures) = fn_regex.captures(line) {
                let fn_name = captures.get(1).unwrap().as_str();
                let code = get_sub_program(lines);
                let code = Program::from_lines(&mut code.iter());
                let args = captures.get(2).unwrap().as_str().split(" ").collect();
                let u_func = UserFunction {
                    code,
                    args,
                };
                println!("added user func: \"{}\"", fn_name);
                user_fns.insert(fn_name, u_func);
            } else if let Some(expression) = Expression::parse(line, &user_fns) {
                program.push(Line::Expression(expression));
            }
        }

        Program {
            program,
            user_fns
        }
    }

    pub fn start(&self) {
        self.run_with(&mut DataStore::new());
    }

    pub fn run_with(&self, data_store: &mut DataStore<'a>) {
        data_store.expand();
        for line in self.program.iter() {
            match line {
                Line::Assignment(var, exp) => {
                    let val = exp.evaluate(data_store, &self.user_fns).unwrap();
                    data_store.put(var, val);
                }
                Line::Expression(exp) => {
                    exp.evaluate(data_store, &self.user_fns);
                }
                Line::Construct(cons) => {
                    cons.apply(data_store, &self.user_fns);
                }
            }
        }
        data_store.contract();
    }
}

pub fn get_sub_program<'a>(lines: &mut Iter<&'a str>) -> Vec<&'a str> {
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