// simulates a stack by making 'layers' using a vec. when a layer is removed, its variables are too.
// if a new var is added, it is added to the top level so the program scopes variables appropriately
pub struct DataStore<'a> {
    vars: Vec<&'a str>,
    vals: Vec<i64>,
    levels: Vec<usize>,
}

impl <'a> DataStore<'a> {
    pub fn new() -> DataStore<'a> {
        DataStore {
            vars: Vec::new(),
            vals: Vec::new(),
            levels: Vec::new(),
        }
    }

    pub fn expand(&mut self) {
        self.levels.push(0);
    }

    pub fn contract(&mut self) {
        for _ in 0..*self.levels.last().unwrap() {
            self.vals.pop();
            self.vars.pop();
        }
        self.levels.pop();
    }

    pub fn put(&mut self, var: &'a str, val: i64) {
        if let Some((i, _)) = self.vars.iter().enumerate().find(|(_, &v)| v == var) {
            self.vals[i] = val;
        }
        else {
            let last_ind = self.levels.len() - 1;
            self.levels[last_ind] += 1;
            self.vars.push(var);
            self.vals.push(val);
        }
    }

    pub fn get(&mut self, var: &'a str) -> Option<i64> {
        for (i, v) in self.vars.iter().enumerate() {
            if *v == var {
                return Some(self.vals[i]);
            }
        }
        None
    }
}
