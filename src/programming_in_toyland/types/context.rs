use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

use super::syntaxtype::Type;

use std::collections::HashMap;

pub struct Context {
    pub vars: HashMap<String, Type>
}

impl Context {
    pub fn new() -> Context {
        Context{ vars: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, nodetype: Type) {
        self.vars.insert(name.to_string(), nodetype);
    }

    pub fn get(&mut self, name: &str) -> Option<Type> {
        match self.vars.get(name) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
}

impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut parts = Vec::new();
        for (key, val) in self.vars.iter() {
            parts.push(format!("key: {0} = val: {1}", key, val))
        };
        let text = parts.join(", ");
        write!(f, "{}", text)
    }
}
