use crate::lib::{DataStore, Expression};

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
    Not(Expression<'a>),
    Print(Expression<'a>),
}

impl <'a> BuiltIns<'a> {
    pub fn get_function(line: &'a str) -> Option<BuiltIns<'a>> {
        if let Some(space) = line.find(" ") {
            let (func, args) = line.split_at(space);
            let args = args.trim();
            let mut args = Expression::evaluate_arguments(args);
            match func {
                "+" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Add(a, b))
                    }
                    _ => panic!("invalid add statement"),
                },
                "-" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Sub(a, b))
                    }
                    _ => panic!("invalid subtract statement"),
                },
                "/" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Div(a, b))
                    }
                    _ => panic!("invalid divide statement"),
                },
                "*" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Mul(a, b))
                    }
                    _ => panic!("invalid multiply statement"),
                },
                "%" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Mod(a, b))
                    }
                    _ => panic!("invalid multiply statement"),
                },
                "==" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Eq(a, b))
                    }
                    _ => panic!("invalid equals statement"),
                },
                "!=" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Neq(a, b))
                    }
                    _ => panic!("invalid not equals statement"),
                },
                ">" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Gt(a, b))
                    }
                    _ => panic!("invalid not equals statement"),
                },
                "<" => match args.len() {
                    2 => {
                        let a = args.remove(0);
                        let b = args.remove(0);
                        return Some(BuiltIns::Lt(a, b))
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

    pub fn apply(&self, data_store: &mut DataStore<'a>) -> Option<i64> {
        match self {
            BuiltIns::Add(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i + j)
            },
            BuiltIns::Div(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i / j)
            },
            BuiltIns::Mul(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i * j)
            },
            BuiltIns::Sub(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i - j)
            },
            BuiltIns::Mod(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(i % j)
            },
            BuiltIns::Eq(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i == j { 1 } else { 0 })
            },
            BuiltIns::Neq(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i == j { 0 } else { 1 })
            },
            BuiltIns::Gt(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i > j { 1 } else { 0 })
            },
            BuiltIns::Lt(i, j) => {
                let i = i.evaluate(data_store).unwrap();
                let j = j.evaluate(data_store).unwrap();
                Some(if i < j { 1 } else { 0 })
            },
            BuiltIns::Not(i) => {
                let i = i.evaluate(data_store).unwrap();
                Some(if i != 0 { 1 } else { 0 })
            },
            BuiltIns::Print(i) => {
                println!("{}", i.evaluate(data_store).unwrap());
                None
            }
        }
    }
}
