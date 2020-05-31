use regex::Regex;

use crate::lib::{Expression, DataStore, interpret};

pub enum Construct {
    If(Expression),
    While(Expression),
    For(String, Expression, Expression)
}

impl Construct {
    pub fn parse(construct: &String, data_store: &mut DataStore) -> Option<Construct> {
        lazy_static! {
            static ref IF_REGEX: Regex = Regex::new(r"^if (.+) \{$").unwrap();
            static ref WHILE_REGEX: Regex = Regex::new(r"^while (.+) \{$").unwrap();
            static ref FOR_REGEX: Regex = Regex::new(r"^for ([a-z]+) (.*) \{$").unwrap();
        }

        if let Some(capture) = IF_REGEX.captures(&construct) {
            let expression = Expression::parse(&capture[1].to_string(), data_store).unwrap();
            Some(Construct::If(expression))
        }
        else if let Some(capture) = WHILE_REGEX.captures(&construct) {
            let expression = Expression::parse(&capture[1].to_string(), data_store).unwrap();
            Some(Construct::While(expression))
        }
        else if let Some(capture) = FOR_REGEX.captures(&construct) {
            let iterating = capture[1].to_string();
            let mut args = Expression::evaluate_arguments(&capture[2].to_string(), data_store);
            match args.len() {
                2 => {
                    let start = args.remove(0);
                    let end = args.remove(0);
                    Some(Construct::For(iterating, start, end))
                }
                _ => panic!("invalid for loop \"{}\"", construct)
            }
        }
        else {
            None
        }
    }

    pub fn apply(&self, lines: Vec<String>, data_store: &mut DataStore) {
        match self {
            Construct::If(expr) => {
                if Expression::evaluate(expr, data_store).unwrap() != 0 {
                    interpret(&lines, data_store);
                }
            },
            Construct::While(expr) => {
                while Expression::evaluate(expr, data_store).unwrap() != 0 {
                    interpret(&lines, data_store);
                }
            },
            Construct::For(var, start, end) => {
                data_store.expand();
                for i in Expression::evaluate(start, data_store).unwrap()..Expression::evaluate(end, data_store).unwrap() {
                    data_store.put(var.to_string(), i);
                    interpret(&lines, data_store);
                }
                data_store.contract();
            }
        }
    }
}