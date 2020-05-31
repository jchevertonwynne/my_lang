use regex::Regex;

use crate::lib::{BuiltIns, DataStore};

#[derive(Debug)]
pub enum Expression {
    Literal(i64),
    Variable(String),
    Operation(String, Vec<Expression>)
}

impl Expression {
    pub fn parse(expression: &String, data_store: &mut DataStore) -> Option<Expression> {
        lazy_static! {
            static ref LITERAL_REGEX: Regex = Regex::new(r"^(\d+)$").unwrap();
            static ref EXPRESSION_REGEX: Regex = Regex::new("^(.+?) (.*)$").unwrap();
        }

        let expression = Expression::remove_outer_brackets(expression.to_string());

        if let Some(capture) = LITERAL_REGEX.captures(&expression) {
            let val = capture[1].parse().unwrap();
            Some(Expression::Literal(val))
        }
        else if let Some(_) =  data_store.get(&expression) {
            Some(Expression::Variable(expression.to_string()))
        }
        else if let Some(capture) = EXPRESSION_REGEX.captures(&expression) {
            let func = capture[1].to_string();
            let args = Expression::evaluate_arguments(&capture[2], data_store);
            Some(Expression::Operation(func, args))
        }
        else {
            None
        }
    }

    pub fn evaluate(&self, data_store: &mut DataStore) -> Option<i64> {
        match self {
            Expression::Literal(val) => Some(*val),
            Expression::Variable(expr) => Some(*data_store.get(expr).unwrap()),
            Expression::Operation(operation, expressions) => {
                BuiltIns::get_function(operation, expressions, data_store)
                    .unwrap()
                    .apply()
            }
        }
    }

    pub fn evaluate_arguments(args: &str, data_store: &mut DataStore) -> Vec<Expression> {
        let mut res: Vec<Expression> = vec![];
        let mut brackets = 0;
        let mut curr_expr = String::new();

        for char in args.chars() {
            curr_expr.push(char);
            match char {
                '(' => brackets += 1,
                ')' => brackets -= 1,
                ' ' => {
                    if brackets == 0 {
                        Expression::parse_append_expr(&mut curr_expr, &mut res, data_store);
                        curr_expr = String::new();
                    }
                }
                _ => ()
            }
        }

        if curr_expr.len() > 0 {
            Expression::parse_append_expr(&mut curr_expr, &mut res, data_store);
        }

        res
    }

    fn parse_append_expr<'a>(curr_expr: &'a mut String, res: &'a mut Vec<Expression>, data_store: &mut DataStore) {
        let expr = Expression::parse(&curr_expr, data_store).unwrap();
        res.push(expr);
    }

    fn remove_outer_brackets(expr: String) -> String {
        let mut expr = expr.trim().to_string();
        let first = expr.chars().nth(0).unwrap();
        let last = expr.chars().last().unwrap();
        if first == '(' && last == ')' {
            expr.remove(expr.len() - 1);
            expr.remove(0);
        }
        expr.trim().to_string()
    }
}