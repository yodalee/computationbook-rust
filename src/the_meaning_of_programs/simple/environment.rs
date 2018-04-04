use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

use super::syntax::Node;

use std::collections::HashMap;

pub struct Environment {
    pub vars: HashMap<String, Box<Node>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment{ vars: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, node: Box<Node>) {
        self.vars.insert(name.to_string(), node);
    }

    pub fn get(&mut self, name: &str) -> Box<Node> {
        match self.vars.get(name) {
            Some(node) => node.clone(),
            None => panic!("Variable {} not found", name),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut parts = Vec::new();
        for (key, val) in self.vars.iter() {
            parts.push(format!("key: {0} = val: {1}", key, val))
        };
        let text = parts.join(", ");
        write!(f, "{}", text)
    }
}
