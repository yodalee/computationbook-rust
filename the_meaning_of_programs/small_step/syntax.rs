use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum Node {
    Number(i64),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Boolean(bool),
    LessThan(Box<Node>, Box<Node>),
}

impl Node {
    pub fn number(value: i64) -> Box<Node> { Box::new(Node::Number(value)) }
    pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Add(left, right)) }
    pub fn multiply(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Multiply(left, right)) }
    pub fn boolean(value: bool) -> Box<Node> { Box::new(Node::Boolean(value)) }
    pub fn lessthan(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::LessThan(left, right)) }

    pub fn reducible(&self) -> bool {
        match *self {
            Node::Number(_) | Node::Boolean(_) => false,
            _ => true,
        }
    }

    pub fn value(&self) -> i64 {
        match *self {
            Node::Number(value) => { value },
            _ => panic!("Type has no value: {}", *self)
        }
    }

    pub fn reduce(&self) -> Box<Node> {
        match *self {
            Node::Add(ref l, ref r) => {
                if l.reducible() {
                    Node::add(l.reduce(), r.clone())
                } else if r.reducible() {
                    Node::add(l.clone(), r.reduce())
                } else {
                    Node::number(l.value() + r.value())
                }
            }
            Node::Multiply(ref l, ref r) => {
                if l.reducible() {
                    Node::add(l.reduce(), r.clone())
                } else if r.reducible() {
                    Node::add(l.clone(), r.reduce())
                } else {
                    Node::number(l.value() * r.value())
                }
            }
            Node::LessThan(ref l, ref r) => {
                if l.reducible() {
                    Node::lessthan(l.reduce(), r.clone())
                } else if r.reducible() {
                    Node::lessthan(l.clone(), r.reduce())
                } else {
                    Node::boolean(l.value() < r.value())
                }
            }
            _ => panic!("Non reducible type found: {}", *self)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Node::Number(value) => write!(f, "{}", value),
            Node::Add(ref l, ref r) => write!(f, "{0} + {1}", l, r),
            Node::Multiply(ref l, ref r) => write!(f, "{0} * {1}", l, r),
            Node::Boolean(value) => write!(f, "{}", value),
            Node::LessThan(ref l, ref r) => write!(f, "{0} < {1}", l, r),
        }
    }
}
