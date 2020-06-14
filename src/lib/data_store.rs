use std::collections::{HashMap, HashSet};

// simulates a stack by making 'layers' using a vec. when a layer is removed, its variables are too.
// if a new var is added, it is added to the top level so the program scopes variables appropriately
pub struct DataStore<'a> {
    vars: HashMap<&'a str, i64>,
    levels: Vec<HashSet<&'a str>>,
}

impl <'a> DataStore<'a> {
    pub fn new() -> DataStore<'a> {
        DataStore {
            vars: HashMap::new(),
            levels: Vec::new(),
        }
    }

    pub fn expand(&mut self) {
        self.levels.push(HashSet::new());
    }

    pub fn contract(&mut self) {
        if let Some(to_remove) = self.levels.pop() {
            for k in to_remove {
                self.vars.remove(&k);
            }
        }
    }

    pub fn put(&mut self, var: &'a str, val: i64) {
        let l = self.levels.len();
        if !self.vars.contains_key(&var) {
            self.levels[l - 1].insert(var);
        }
        self.vars.insert(var, val);
    }

    pub fn get(&mut self, var: &'a str) -> Option<&i64> {
        self.vars.get(var)
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        println!("Data Store:");
        for (k, v) in &self.vars {
            println!("{}: {}", k, v);
        }
    }
}
