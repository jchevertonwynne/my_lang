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
    program: Vec<Line<'a>>,
}

impl<'a> Program<'a> {
    pub fn from_lines(lines: &mut Iter<&'a str>, user_fns: &mut HashMap<&'a str, UserFunction<'a>>) -> Program<'a> {
        let assignment_regex = Regex::new(r"^([a-z_]+): (.+)$").unwrap();
        let fn_regex = Regex::new(r"^func ([a-z_]+) (.+) \{$").unwrap();

        let mut program = Vec::new();

        while let Some(&line) = lines.next() {
            if line.len() == 0 {
                continue;
            }

            // program moves back to outer scope that called it
            if line == "}" {
                break;
            }

            // an assignment will be of the form `var: EXPRESSION`
            if let Some(captures) = assignment_regex.captures(line) {
                let var = captures.get(1).unwrap().as_str();
                let args = captures.get(2).unwrap().as_str();
                let exp = Expression::parse(args, &user_fns).unwrap();
                program.push(Line::Assignment(var, exp));
            } 
            // check if line matches any of the constucts - if/while/for
            else if let Some(construct) = Construct::parse(line, lines, user_fns) {
                program.push(Line::Construct(construct));
            } 
            // function declaration of form `func func_name (a r g s) {`
            else if let Some(captures) = fn_regex.captures(line) {
                let fn_name = captures.get(1).unwrap().as_str();
                let code = get_sub_program(lines);
                let code = Program::from_lines(&mut code.iter(), user_fns);
                let args = captures.get(2).unwrap().as_str().split(" ").collect();
                let u_func = UserFunction {
                    code,
                    args,
                };
                user_fns.insert(fn_name, u_func);
                println!("added user func: \"{}\"", fn_name);
            } 
            // xpressions can be literals, built in funcs, previously defined user funcs or variables.
            // non-matches are currently assumed to be var names
            else if let Some(expression) = Expression::parse(line, &user_fns) {
                program.push(Line::Expression(expression));
            }
            else {
                panic!(format!("unexpected input : \"{}\"", line))
            }
        }

        Program {
            program
        }
    }

    pub fn start(&self, user_fns: &HashMap<&'a str, UserFunction<'a>>) {
        self.run_with(&mut DataStore::new(), user_fns);
    }

    pub fn run_with(&self, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) {
        data_store.expand();
        for line in self.program.iter() {
            match line {
                Line::Assignment(var, exp) => {
                    let val = exp.evaluate(data_store, user_fns).unwrap();
                    data_store.put(var, val);
                }
                Line::Expression(exp) => {
                    exp.evaluate(data_store, user_fns);
                }
                Line::Construct(cons) => {
                    cons.apply(data_store, user_fns);
                }
            }
        }
        data_store.contract();
    }
}

// get all following lines from the inner level of indentation (if/while/for/function code)
pub fn get_sub_program<'a>(lines: &mut Iter<&'a str>) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
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