use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

use environment::Environment;

#[derive(Clone)]
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

    pub fn reducible(&self) -> bool {
        match *self {
            Node::Number(_) | Node::Boolean(_) | Node::DoNothing => false,
            _ => true,
        }
    }

    pub fn value(&self) -> i64 {
        match *self {
            Node::Number(value) => { value },
            _ => panic!("Type has no value: {}", *self)
        }
    }

    pub fn eval(&self) -> bool {
        match *self {
            Node::Boolean(b) => { b },
            _ => panic!("Type cannot eval to boolean {}", *self)
        }
    }

    pub fn reduce(&self, environment: &mut Environment) -> Box<Node> {
        match *self {
            Node::Add(ref l, ref r) => {
                if l.reducible() {
                    Node::add(l.reduce(environment), r.clone())
                } else if r.reducible() {
                    Node::add(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() + r.value())
                }
            }
            Node::Multiply(ref l, ref r) => {
                if l.reducible() {
                    Node::multiply(l.reduce(environment), r.clone())
                } else if r.reducible() {
                    Node::multiply(l.clone(), r.reduce(environment))
                } else {
                    Node::number(l.value() * r.value())
                }
            }
            Node::LessThan(ref l, ref r) => {
                if l.reducible() {
                    Node::lessthan(l.reduce(environment), r.clone())
                } else if r.reducible() {
                    Node::lessthan(l.clone(), r.reduce(environment))
                } else {
                    Node::boolean(l.value() < r.value())
                }
            }
            Node::Variable(ref name) => {
                environment.get(&name)
            }
            Node::Assign(ref name, ref expr) => {
                if expr.reducible() {
                    Node::assign(name, expr.reduce(environment))
                } else {
                    environment.add(name, expr.clone());
                    Node::donothing()
                }
            }
            Node::If(ref condition, ref consequence, ref alternative) => {
                if condition.reducible() {
                    Node::if_cond_else(condition.reduce(environment), consequence.clone(), alternative.clone())
                } else {
                    if condition.eval() {
                        consequence.clone()
                    } else {
                        alternative.clone()
                    }
                }
            }
            Node::Sequence(ref head, ref more) => {
                match **head {
                    Node::DoNothing => more.clone(),
                    _ => Node::sequence(head.reduce(environment), more.clone()),
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
            Node::Variable(ref name) => write!(f, "{}", name),
            Node::DoNothing => write!(f, "do-nothing"),
            Node::Assign(ref name, ref expr) => write!(f, "{0} = {1}", name, expr),
            Node::If(ref condition, ref consequence, ref alternative) => write!(f, "if ({0}) {1} else {2}", condition, consequence, alternative),
            Node::Sequence(ref head, ref more) => write!(f, "{0}; {1}", head, more),
        }
    }
}
