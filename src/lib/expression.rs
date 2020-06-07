use regex::Regex;

use crate::lib::{BuiltIns, DataStore};

#[derive(Debug)]
pub enum Expression<'a> {
    Literal(i64),
    Variable(&'a str),
    Operation(Box<BuiltIns<'a>>),
}

impl <'a> Expression<'a> {
    pub fn parse(expression: &'a str) -> Option<Expression<'a>> {
        let literal_regex = Regex::new(r"^(-?\d+)$").unwrap();

        let expression = Expression::remove_outer_brackets(expression);

        if let Some(capture) = literal_regex.captures(&expression) {
            let val = capture[1].parse().unwrap();
            Some(Expression::Literal(val))
        }
        else if let Some(built_in) = BuiltIns::get_function(&expression) {
            Some(Expression::Operation(Box::from(built_in)))
        }
        else {
            Some(Expression::Variable(expression))
        }
    }

    pub fn evaluate(&self, data_store: &mut DataStore<'a>) -> Option<i64> {
        match self {
            Expression::Literal(val) => Some(*val),
            Expression::Variable(expr) => Some(*data_store.get(expr).unwrap()),
            Expression::Operation(op) => op.apply(data_store)
        }
    }

    pub fn evaluate_arguments(args: &'a str) -> Vec<Expression<'a>> {
        let mut res: Vec<Expression> = vec![];
        let mut brackets = 0;
        let mut start = 0;
        let mut end = 0;

        const OPEN_BRACKET: u8 = 40;
        const CLOSE_BRACKET: u8 = 41;
        const SPACE: u8 = 32;

        while end < args.len() {
            match args.as_bytes()[end] {
                OPEN_BRACKET => brackets += 1,
                CLOSE_BRACKET => brackets -= 1,
                SPACE => {
                    if brackets == 0 {
                        let expr = Expression::parse(&args[start..end]).unwrap();
                        res.push(expr);
                        start = end;
                    }
                }
                _ => (),
            }
            end += 1;
        }

        if end - start > 0 {
            let expr = Expression::parse(&args[start..end]).unwrap();
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
            return expr.trim()
        }

        expr.trim()
    }
}
