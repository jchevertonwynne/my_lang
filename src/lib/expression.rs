use regex::Regex;

use crate::lib::{BuiltIns, DataStore};
use crate::lib::user_function::UserFunction;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression<'a> {
    Literal(i64),
    Variable(&'a str),
    Operation(Box<BuiltIns<'a>>),
    UserOperation(&'a str, Vec<Expression<'a>>)
}

impl<'a> Expression<'a> {
    pub fn parse(expression: &'a str, user_funcs: &HashMap<&'a str, UserFunction<'a>>) -> Option<Expression<'a>> {
        let literal_regex = Regex::new(r"^(-?\d+)$").unwrap();

        let expression = Expression::remove_outer_brackets(expression);

        if let Some(capture) = literal_regex.captures(&expression) {
            let val = capture[1].parse().unwrap();
            Some(Expression::Literal(val))
        } else if let Some(built_in) = BuiltIns::get_function(&expression, user_funcs) {
            Some(Expression::Operation(Box::from(built_in)))
        } else if let Some(user_func) = is_user_function_call(expression, user_funcs) {
            let space_index = expression.find(" ").unwrap();
            let args = expression[space_index..].trim();
            let args = Expression::evaluate_arguments(args, user_funcs);
            Some(Expression::UserOperation(user_func, args))
        } else {
            Some(Expression::Variable(expression))
        }
    }

    pub fn evaluate(&self, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<i64> {
        match self {
            Expression::Literal(val) => Some(*val),
            Expression::Variable(var) => {
                let val = data_store.get(var);
                Some(*val.unwrap())
            }
            Expression::Operation(op) => op.apply(data_store, user_fns),
            Expression::UserOperation(func, args) => {
                let func = user_fns.get(func).unwrap();
                func.apply(args, data_store, user_fns)
            }
        }
    }

    pub fn evaluate_arguments(args: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Vec<Expression<'a>> {
        let mut res: Vec<Expression> = vec![];
        let mut brackets = 0;
        let mut start = 0;
        let mut end = 0;

        const OPEN_BRACKET: u8 = '(' as u8;
        const CLOSE_BRACKET: u8 = ')' as u8;
        const SPACE: u8 = ' ' as u8;

        let bytes = args.as_bytes();

        while end < args.len() {
            match bytes[end] {
                OPEN_BRACKET => brackets += 1,
                CLOSE_BRACKET => brackets -= 1,
                SPACE => {
                    if brackets == 0 {
                        let expr = Expression::parse(&args[start..end], user_fns).unwrap();
                        res.push(expr);
                        start = end;
                    }
                }
                _ => (),
            }
            end += 1;
        }

        if start != end {
            let expr = Expression::parse(&args[start..], user_fns).unwrap();
            res.push(expr);
        }

        res
    }

    fn remove_outer_brackets(expr: &'a str) -> &'a str {
        let mut expr = expr.trim();
        let first = expr.chars().nth(0).unwrap();
        let last = expr.chars().last().unwrap();
        if first == '(' && last == ')' {
            expr = &expr[1..expr.len() - 1];
            return expr.trim();
        }

        expr.trim()
    }
}

fn is_user_function_call<'a>(line: &'a str, user_funcs: &HashMap<&'a str, UserFunction<'a>>) -> Option<&'a str> {
    match line.find(' ') {
        Some(ind) => {
            let f_name = &line[..ind];
            for &func_name in user_funcs.keys() {
                if func_name.eq(f_name) {
                    return Some(func_name)
                }
            }
            return None
        }
        _ => None
    }
}