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
    Ternary(Expression<'a>, Expression<'a>, Expression<'a>),
    Not(Expression<'a>),
    Print(Vec<Expression<'a>>),
    Printa(Vec<Expression<'a>>),
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
                        _ => panic!("invalid gt statement"),
                    },
                    "<" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Lt(a, b))
                        }
                        _ => panic!("invalid lt statement"),
                    },
                    ">=" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Ge(a, b))
                        }
                        _ => panic!("invalid ge statement"),
                    },
                    "<=" => match args.len() {
                        2 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            Some(BuiltIns::Le(a, b))
                        }
                        _ => panic!("invalid le statement"),
                    },
                    "?" => match args.len() {
                        3 => {
                            let a = args.remove(0);
                            let b = args.remove(0);
                            let c = args.remove(0);
                            Some(BuiltIns::Ternary(a, b, c))
                        }
                        _ => panic!("invalid ternary"),
                    },
                    "!" => match args.len() {
                        1 => Some(BuiltIns::Not(args.remove(0))),
                        _ => panic!("invalid not statement"),
                    },
                    "print" => return Some(BuiltIns::Print(args)),
                    "printa" => return Some(BuiltIns::Printa(args)),
                    _ => return None,
                }
            },
            None => {
                match line {
                    "print" => Some(BuiltIns::Print(Vec::new())),
                    "printa" => Some(BuiltIns::Printa(Vec::new())),
                    _ => None
                }
            }
        }
    }

    pub fn apply(&self, data_store: &mut DataStore<'a>) -> Option<i64> {
        match self {
            BuiltIns::Add(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i + j)
            }
            BuiltIns::Div(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i / j)
            }
            BuiltIns::Mul(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i * j)
            }
            BuiltIns::Sub(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i - j)
            }
            BuiltIns::Mod(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i % j)
            }
            BuiltIns::Eq(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i == j { 1 } else { 0 })
            }
            BuiltIns::Neq(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i == j { 0 } else { 1 })
            }
            BuiltIns::Gt(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i > j { 1 } else { 0 })
            }
            BuiltIns::Lt(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i < j { 1 } else { 0 })
            }
            BuiltIns::Ge(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i >= j { 1 } else { 0 })
            }
            BuiltIns::Le(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i <= j { 1 } else { 0 })
            }
            BuiltIns::Ternary(a, b, c) => {
                let expr = a.evaluate(data_store).unwrap();
                if expr != 0 {
                    Some(b.evaluate(data_store).unwrap())
                }
                else {
                    Some(c.evaluate(data_store).unwrap())
                }
            }
            BuiltIns::Not(i) => {
                let i = i.evaluate(data_store).unwrap();
                Some(if i != 0 { 1 } else { 0 })
            }
            BuiltIns::Print(args) => {
                let expr_strings: Vec<String> = args.iter()
                    .map(|v| v.evaluate(data_store).unwrap().to_string())
                    .collect();
                println!("{}", expr_strings.join(" "));
                None
            }
            BuiltIns::Printa(args) => {
                let as_string: String = args.iter()
                    .map(|v| std::char::from_u32(v.evaluate(data_store).unwrap() as u32).unwrap())
                    .collect();
                println!("{}", as_string);
                None
            }
        }
    }

    pub fn optimise(&'a self, user_fns: &'a HashMap<&'a str, UserFunction<'a>>) -> BuiltIns<'a> {
        match self {
            BuiltIns::Add(a, b) => BuiltIns::Add(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Sub(a, b) => BuiltIns::Sub(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Mul(a, b) => BuiltIns::Mul(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Div(a, b) => BuiltIns::Div(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Mod(a, b) => BuiltIns::Mod(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Eq(a, b) => BuiltIns::Eq(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Neq(a, b) => BuiltIns::Neq(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Lt(a, b) => BuiltIns::Lt(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Gt(a, b) => BuiltIns::Gt(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Le(a, b) => BuiltIns::Le(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Ge(a, b) => BuiltIns::Ge(a.optimise(user_fns), b.optimise(user_fns)),
            BuiltIns::Ternary(a, b, c) => BuiltIns::Ternary(a.optimise(user_fns), b.optimise(user_fns), c.optimise(user_fns)),
            BuiltIns::Not(a) => BuiltIns::Not(a.optimise(user_fns)),
            BuiltIns::Print(args) => {
                let args = args.iter()
                    .map(|arg| arg.optimise(user_fns))
                    .collect();
                BuiltIns::Print(args)
            }
            BuiltIns::Printa(args) => {
                let args = args.iter()
                    .map(|arg| arg.optimise(user_fns))
                    .collect();
                BuiltIns::Printa(args)
            }
        }
    }
}
