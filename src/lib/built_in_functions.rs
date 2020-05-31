use crate::lib::{Expression, DataStore};

pub enum BuiltIns {
    Add(i64, i64),
    Sub(i64, i64),
    Mul(i64, i64),
    Div(i64, i64),
    Eq(i64, i64),
    Neq(i64, i64),
    Not(i64),
    Print(i64)
}

impl BuiltIns {
    pub fn get_function(line: &str, args: &Vec<Expression>, data_store: &mut DataStore) -> Option<BuiltIns> {
        match line {
            "+" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Add(a, b))
                    }
                    _ => panic!("invalid add statement")
                }
            }
            "-" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Sub(a, b))
                    }
                    _ => panic!("invalid subtract statement")
                }
            }
            "/" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Div(a, b))
                    }
                    _ => panic!("invalid divide statement")
                }
            }
            "*" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Mul(a, b))
                    }
                    _ => panic!("invalid multiply statement")
                }
            }
            "==" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Eq(a, b))
                    }
                    _ => panic!("invalid equals statement")
                }
            }
            "!=" => {
                match args.len() {
                    2 => {
                        let a = Expression::evaluate(&args[0], data_store).unwrap();
                        let b = Expression::evaluate(&args[1], data_store).unwrap();
                        Some(BuiltIns::Neq(a, b))
                    }
                    _ => panic!("invalid not equals statement")
                }
            }
            "!" => {
                match args.len() {
                    1 => Some(BuiltIns::Not(Expression::evaluate(&args[0], data_store).unwrap())),
                    _ => panic!("invalid not statement")
                }
            }
            "print" => {
                match args.len() {
                    1 => Some(BuiltIns::Print(Expression::evaluate(&args[0], data_store).unwrap())),
                    _ => panic!("invalid print statement")
                }
            }
            _ => None
        }
    }

    pub fn apply(&self) -> Option<i64> {
        match self {
            BuiltIns::Add(i, j) => Some(i + j),
            BuiltIns::Div(i, j) => Some(i / j),
            BuiltIns::Mul(i, j) => Some(i * j),
            BuiltIns::Sub(i, j) => Some(i - j),
            BuiltIns::Eq(i, j) => Some(if i == j { 1 } else { 0 }),
            BuiltIns::Neq(i, j) => Some(if i == j { 0 } else { 1 }),
            BuiltIns::Not(i) => Some(if *i != 0 { 1 } else { 0 }),
            BuiltIns::Print(i) => {
                println!("{}", i);
                None
            }
        }
    }
}