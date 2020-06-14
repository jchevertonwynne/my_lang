use crate::lib::{Program, Expression, DataStore};
use std::collections::HashMap;

// a function consists of its code and the names of the arguments you can pass it
pub struct UserFunction<'a> {
    pub code: Program<'a>,
    pub args: Vec<&'a str>,
}

impl<'a> UserFunction<'a> {
    // 
    pub fn apply(&self, vars: &Vec<Expression<'a>>, data_store: &mut DataStore<'a>, user_fns: &HashMap<&'a str, UserFunction<'a>>) -> Option<i64> {
        if self.args.len() != vars.len() {
            panic!("please don't give me the wrong number of arguments")
        }
        let mut function_data_store = DataStore::new();
        function_data_store.expand();
        function_data_store.put("res", 0);
        // get arg values from outer program and load into user function with arg names
        self.args.iter().zip(vars)
            .for_each(|(k, v)| {
                let val = v.evaluate(data_store, user_fns).unwrap();
                function_data_store.put(k, val);
            });
        self.code.run_with(&mut function_data_store, user_fns);
        let result = *function_data_store.get("res").unwrap();
        Some(result)
    }
}