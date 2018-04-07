use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

#[derive(Debug,PartialEq,Clone)]
pub enum Node {
    Number(i64),
    Add(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Boolean(bool),
    LessThan(Box<Node>, Box<Node>),
    Variable(String),
    DoNothing,
    Assign(String, Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    Sequence(Box<Node>, Box<Node>),
    While(Box<Node>, Box<Node>),
}

impl Node {
    pub fn number(value: i64) -> Box<Node> { Box::new(Node::Number(value)) }
    pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Add(left, right)) }
    pub fn multiply(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::Multiply(left, right)) }
    pub fn boolean(value: bool) -> Box<Node> { Box::new(Node::Boolean(value)) }
    pub fn lessthan(left: Box<Node>, right: Box<Node>) -> Box<Node> { Box::new(Node::LessThan(left, right)) }
    pub fn variable(name: &str) -> Box<Node> { Box::new(Node::Variable(name.to_string())) }
    pub fn donothing() -> Box<Node> { Box::new(Node::DoNothing) }
    pub fn assign(name: &str, expr: Box<Node>) -> Box<Node> { Box::new(Node::Assign(name.to_string(), expr)) }
    pub fn if_cond_else(condition: Box<Node>, consequence: Box<Node>, alternative: Box<Node>) -> Box<Node> { Box::new(Node::If(condition, consequence, alternative)) }
    pub fn sequence(head: Box<Node>, more: Box<Node>) -> Box<Node> { Box::new(Node::Sequence(head, more)) }
    pub fn while_node(cond: Box<Node>, body: Box<Node>) -> Box<Node> { Box::new(Node::While(cond, body)) }

    pub fn value(&self) -> i64 {
        match *self {
            Node::Number(value) => { value },
            _ => panic!("Type has no value: {}", *self)
        }
    }

    pub fn condition(&self) -> bool {
        match *self {
            Node::Boolean(b) => { b },
            _ => panic!("Type cannot eval to boolean {}", *self)
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
            Node::Variable(ref name) => write!(f, "{}", name),
            Node::DoNothing => write!(f, "do-nothing"),
            Node::Assign(ref name, ref expr) => write!(f, "{0} = {1}", name, expr),
            Node::If(ref condition, ref consequence, ref alternative) => write!(f, "if ({0}) {1} else {2}", condition, consequence, alternative),
            Node::Sequence(ref head, ref more) => write!(f, "{0}; {1}", head, more),
            Node::While(ref cond, ref body) => write!(f, "while ({0}) {1}", cond, body)
        }
    }
}
