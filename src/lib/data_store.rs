use std::collections::{HashMap, HashSet};

pub struct DataStore {
    vars: HashMap<String, i64>,
    levels: Vec<HashSet<String>>
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore{
            vars: HashMap::new(),
            levels: vec![]
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

    pub fn put(&mut self, var: String, val: i64) {
        let l = self.levels.len();
        if !self.vars.contains_key(&var) {
            self.levels[l - 1].insert(var.to_string());
        }
        self.vars.insert(var, val);
    }

    pub fn get(&mut self, var: &String) -> Option<&i64> {
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