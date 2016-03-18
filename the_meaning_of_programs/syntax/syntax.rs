use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

pub enum Node {
    Number(i64),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
}

impl Node {
    pub fn number(value: i64) -> Box<Node> { Box::new(Node::Number(value)) }
    pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Add(left, right)) }
    pub fn multiply(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Multiply(left, right)) }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Node::Number(value) => write!(f, "{}", value),
            Node::Add(ref l, ref r) => write!(f, "{0} + {1}", l, r),
            Node::Multiply(ref l, ref r) => write!(f, "{0} * {1}", l, r),
        }
    }
}
