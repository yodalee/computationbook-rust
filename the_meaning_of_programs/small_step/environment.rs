use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

use std::collections::HashMap;

use syntax::Node;

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
