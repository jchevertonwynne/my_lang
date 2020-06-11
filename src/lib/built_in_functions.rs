use crate::lib::{DataStore, Expression};
use std::collections::HashMap;
use crate::lib::user_function::UserFunction;

#[derive(Debug)]
pub enum BuiltIns<'a> {
    Add(Expression<'a>, Expression<'a>),
    Sub(Expression<'a>, Expression<'a>),
    Mul(Expression<'a>, Expression<'a>),
    Div(Expression<'a>, Expression<'a>),
    Mod(Expression<'a>, Expression<'a>),
    Eq(Expression<'a>, Expression<'a>),
    Neq(Expression<'a>, Expression<'a>),
    Lt(Expression<'a>, Expression<'a>),
    Gt(Expression<'a>, Expression<'a>),
    Le(Expression<'a>, Expression<'a>),
    Ge(Expression<'a>, Expression<'a>),
    Not(Expression<'a>),
    Print(Expression<'a>),
}

impl<'a> BuiltIns<'a> {
    pub fn get_function(line: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<BuiltIns<'a>> {
        if let Some(space) = line.find(" ") {
            let (func, args) = line.split_at(space);
            let args = args.trim();
            let mut args = Expression::evaluate_arguments(args, user_fns);
            match func {
                "+" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Add(a, b));
                    }
                    _ => panic!("invalid add statement"),
                },
                "-" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Sub(a, b));
                    }
                    _ => panic!("invalid subtract statement"),
                },
                "/" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Div(a, b));
                    }
                    _ => panic!("invalid divide statement"),
                },
                "*" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Mul(a, b));
                    }
                    _ => panic!("invalid multiply statement"),
                },
                "%" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Mod(a, b));
                    }
                    _ => panic!("invalid multiply statement"),
                },
                "==" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Eq(a, b));
                    }
                    _ => panic!("invalid equals statement"),
                },
                "!=" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Neq(a, b));
                    }
                    _ => panic!("invalid not equals statement"),
                },
                ">" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Gt(a, b));
                    }
                    _ => panic!("invalid not equals statement"),
                },
                "<" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Lt(a, b));
                    }
                    _ => panic!("invalid not equals statement"),
                },
                ">=" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Ge(a, b));
                    }
                    _ => panic!("invalid not equals statement"),
                },
                "<=" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Le(a, b));
                    }
                    _ => panic!("invalid not equals statement"),
                },
                "!" => match args.len() {
                    1 => return Some(BuiltIns::Not(args.remove(0))),
                    _ => panic!("invalid not statement"),
                },
                "print" => match args.len() {
                    1 => return Some(BuiltIns::Print(args.remove(0))),
                    _ => panic!("invalid print statement"),
                },
                _ => return None,
            }
        }
        None
    }

    pub fn apply(&self, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<i64> {
        match self {
            BuiltIns::Add(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(i + j)
            }
            BuiltIns::Div(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(i / j)
            }
            BuiltIns::Mul(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(i * j)
            }
            BuiltIns::Sub(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(i - j)
            }
            BuiltIns::Mod(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(i % j)
            }
            BuiltIns::Eq(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i == j { 1 } else { 0 })
            }
            BuiltIns::Neq(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i == j { 0 } else { 1 })
            }
            BuiltIns::Gt(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i > j { 1 } else { 0 })
            }
            BuiltIns::Lt(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i < j { 1 } else { 0 })
            }
            BuiltIns::Ge(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i >= j { 1 } else { 0 })
            }
            BuiltIns::Le(i, j) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                let j = j.evaluate(data_store, user_fns).unwrap();
                Some(if i <= j { 1 } else { 0 })
            }
            BuiltIns::Not(i) => {
                let i = i.evaluate(data_store, user_fns).unwrap();
                Some(if i != 0 { 1 } else { 0 })
            }
            BuiltIns::Print(i) => {
                println!("{}", i.evaluate(data_store, user_fns).unwrap());
                None
            }
        }
    }
}
