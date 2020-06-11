use crate::lib::{Program, Expression, DataStore};
use std::collections::HashMap;

#[derive(Debug)]
pub struct UserFunction<'a> {
    pub code: Program<'a>,
    pub args: Vec<&'a str>,
}

impl<'a> UserFunction<'a> {
    pub fn apply(&self, vars: &Vec<Expression<'a>>, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<i64> {
        data_store.expand();
        data_store.put("res", 0);
        self.args.iter().zip(vars)
            .for_each(|(k, v)| {
                let val = v.evaluate(data_store, user_fns).unwrap();
                data_store.put(k, val);
            });
        self.code.run_with(data_store);
        let result = *data_store.get("res").unwrap();
        data_store.contract();
        Some(result)
    }
}