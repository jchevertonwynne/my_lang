use crate::lib::{DataStore, Expression};

pub enum BuiltIns {
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Mul(Expression, Expression),
    Div(Expression, Expression),
    Eq(Expression, Expression),
    Neq(Expression, Expression),
    Not(Expression),
    Print(Expression),
}

impl BuiltIns {
    pub fn get_function(line: &String) -> Option<BuiltIns> {
        if let Some(space) = line.find(" ") {
            let (func, args) = line.split_at(space);
            let args = args.trim();
            let mut args = Expression::evaluate_arguments(&args);
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

    pub fn apply(&self, data_store: &mut DataStore) -> Option<i64> {
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
