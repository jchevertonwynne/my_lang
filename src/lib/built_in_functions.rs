use crate::lib::{DataStore, Expression};
use std::collections::HashMap;
use crate::lib::user_function::UserFunction;

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
    Print(Vec<Expression<'a>>),
}

// defines standard math/logic operators and print
impl<'a> BuiltIns<'a> {
    pub fn get_function(line: &'a str, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<BuiltIns<'a>> {
        match line.find(" ") {
            Some(space) => {
                let (func, args) = line.split_at(space);
                let args = args.trim();
                let mut args = Expression::evaluate_arguments(args, user_fns);
                match func {
                    "+" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Add(a, b))
                        }
                        _ => panic!("invalid add statement"),
                    },
                    "-" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Sub(a, b))
                        }
                        _ => panic!("invalid subtract statement"),
                    },
                    "/" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Div(a, b))
                        }
                        _ => panic!("invalid divide statement"),
                    },
                    "*" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Mul(a, b))
                        }
                        _ => panic!("invalid multiply statement"),
                    },
                    "%" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Mod(a, b))
                        }
                        _ => panic!("invalid multiply statement"),
                    },
                    "==" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Eq(a, b))
                        }
                        _ => panic!("invalid equals statement"),
                    },
                    "!=" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Neq(a, b))
                        }
                        _ => panic!("invalid not equals statement"),
                    },
                    ">" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Gt(a, b))
                        }
                        _ => panic!("invalid not equals statement"),
                    },
                    "<" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Lt(a, b))
                        }
                        _ => panic!("invalid not equals statement"),
                    },
                    ">=" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Ge(a, b))
                        }
                        _ => panic!("invalid not equals statement"),
                    },
                    "<=" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Le(a, b))
                        }
                        _ => panic!("invalid not equals statement"),
                    },
                    "!" => match args.len() {
                        1 => Some(BuiltIns::Not(args.remove(0))),
                        _ => panic!("invalid not statement"),
                    },
                    "print" => return Some(BuiltIns::Print(args)),
                    _ => return None,
                }
            },
            None => {
                match line {
                    "print" => Some(BuiltIns::Print(Vec::new())),
                    _ => None
                }
            }
        }
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
            BuiltIns::Print(args) => {
                let expr_strings: Vec<String> = args.iter()
                    .map(|v| v.evaluate(data_store, user_fns).unwrap().to_string())
                    .collect();
                println!("{}", expr_strings.join(" "));
                None
            }
        }
    }
}
