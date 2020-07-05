use crate::lib::{Program, Expression, DataStore};
use std::collections::HashMap;

// a function consists of its code and the names of the arguments you can pass it
#[derive(Debug)]
pub struct UserFunction<'a> {
    pub code: Program<'a>,
    pub args: Vec<&'a str>,
}

impl<'a> UserFunction<'a> {
    // 
    pub fn apply(&self, vars: &Vec<Expression<'a>>, data_store: &mut DataStore<'a>) -> Option<i64> {
        if self.args.len() != vars.len() {
            panic!("please don't give me the wrong number of arguments")
        }
        let mut function_data_store = DataStore::new();
        function_data_store.expand();
        function_data_store.put("res", 0);
        // get arg values from outer program and load into user function with arg names
        self.args.iter().zip(vars)
            .for_each(|(k, val)| {
                let val = val.evaluate(data_store).unwrap();
                function_data_store.put(k, val);
            });
        self.code.run_with(&mut function_data_store);
        function_data_store.get("res")
    }

    pub fn optimise(&'a self, user_fns: &'a HashMap<&'a str, UserFunction<'a>>) -> UserFunction<'a> {
        UserFunction{
            code: self.code.optimise(user_fns),
            args: self.args.clone()
        }
    }
}