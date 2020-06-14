use regex::Regex;

use crate::lib::{BuiltIns, DataStore};
use crate::lib::user_function::UserFunction;
use std::collections::HashMap;

pub enum Expression<'a> {
    Literal(i64),
    Variable(&'a str),
    BuiltInFunction(Box<BuiltIns<'a>>),
    UserFunction(&'a str, Vec<Expression<'a>>)
}

impl<'a> Expression<'a> {
    // an exression can be a literal - 1, 3, -4. any valid i64
    // or a built in func - see built_in_functions.rs
    // or a user func - as defined by `func func_name (v a r s) {`. must have been declared prior to evaluation of its call
    // else assumed to be a variable name
    pub fn parse(expression: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<Expression<'a>> {
        let literal_regex = Regex::new(r"^(-?\d+)$").unwrap();

        let expression = Expression::remove_outer_brackets(expression);

        if let Some(capture) = literal_regex.captures(&expression) {
            let val = capture[1].parse().unwrap();
            Some(Expression::Literal(val))
        } else if let Some(built_in) = BuiltIns::get_function(&expression, user_fns) {
            Some(Expression::BuiltInFunction(Box::from(built_in)))
        } else if let Some(user_fn) = is_user_function_call(expression, user_fns) {
            let space_index = expression.find(" ").unwrap();
            let args = expression[space_index..].trim();
            let args = Expression::evaluate_arguments(args, user_fns);
            Some(Expression::UserFunction(user_fn, args))
        } else {
            Some(Expression::Variable(expression))
        }
    }

    // take an expression and find its value
    pub fn evaluate(&self, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<i64> {
        match self {
            Expression::Literal(literal) => Some(*literal),
            Expression::Variable(variable) => {
                let val = data_store.get(variable);
                Some(*val.unwrap())
            }
            Expression::BuiltInFunction(operation) => operation.apply(data_store, user_fns),
            Expression::UserFunction(func, args) => {
                let func = user_fns.get(func).unwrap();
                func.apply(args, data_store, user_fns)
            }
        }
    }

    // takes a string and seperates it into its individual expressions. these are then individually parsed
    // "1 (+ 2 3) 4" => ["1", "(+ 2 3)", "4"]
    pub fn evaluate_arguments(args: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Vec<Expression<'a>> {
        let mut res: Vec<Expression> = Vec::new();
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

        // something left to do
        if start != end {
            let expr = Expression::parse(&args[start..], user_fns).unwrap();
            res.push(expr);
        }

        res
    }

    // "(+ 2 3)" becomes "+ 2 3"
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

// checks map of user functions for one by the name of the first word of the line. if it's there, extract its name
fn is_user_function_call<'a>(line: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<&'a str> {
    match line.find(' ') {
        Some(ind) => {
            let f_name = &line[..ind];
            for &func_name in user_fns.keys() {
                if func_name.eq(f_name) {
                    return Some(func_name)
                }
            }
            return None
        }
        _ => None
    }
}