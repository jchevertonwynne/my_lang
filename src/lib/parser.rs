use regex::{Regex};

use crate::lib::DataStore;
use crate::lib::Expression;
use crate::lib::Construct;
use core::slice::Iter;

pub fn interpret(lines: &Vec<String>, data_store: &mut DataStore) {
    lazy_static! {
        static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"^([a-z]+): (.+)$").unwrap();
    }

    data_store.expand();

    let mut lines: Iter<String> = lines.iter();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }

        if line == "}" {
            break;
        }

        if let Some(captures) = ASSIGNMENT_REGEX.captures(line) {
            let var = captures[1].to_string();
            let args = captures[2].to_string();
            let exp = Expression::parse(&args, data_store).unwrap();
            let val = Expression::evaluate(&exp, data_store).unwrap();
            data_store.put(var, val);
        }
        else if let Some(construct) = Construct::parse(&line.to_string(), data_store) {
            let sub_lines = get_sub_program(&mut lines);
            construct.apply(sub_lines, data_store);
        }
        else if let Some(expression) = Expression::parse(&line.to_string(), data_store) {
            expression.evaluate(data_store);
        }
    }

    data_store.contract();
}

fn get_sub_program(lines: &mut Iter<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut brackets = 1;

    while let Some(line) = lines.next() {
        if line.eq(&String::from("{")) {
            brackets += 1;
        }
        else if line.eq(&String::from("}"))  {
            brackets -= 1;
            if brackets == 0 {
                return res
            }
        }

        res.push(line.to_string());
    }

    panic!("unclosed pair of squiggly brackets");
}