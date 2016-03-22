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
            Node::Boolean(value) => { value },
            _ => panic!("Type cannot be boolean: {}", *self)
        }
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Box<Node> {
        match *self {
            Node::Number(v) => { Node::number(v) }
            Node::Boolean(v) => { Node::boolean(v) }
            Node::DoNothing => { Node::donothing() }
            Node::Add(ref l, ref r) => {
                Node::number(l.evaluate(environment).value() + r.evaluate(environment).value())
            }
            Node::Multiply(ref l, ref r) => {
                Node::number(l.evaluate(environment).value() * r.evaluate(environment).value())
            }
            Node::LessThan(ref l, ref r) => {
                Node::boolean(l.evaluate(environment).value() < r.evaluate(environment).value())
            }
            Node::Variable(ref name) => {
                environment.get(&name)
            }
            Node::Assign(ref name, ref expr) => {
                let reduce = expr.evaluate(environment);
                environment.add(name, reduce.clone());
                Node::donothing()
            }
            Node::If(ref condition, ref consequence, ref alternative) => {
                if condition.evaluate(environment).condition() {
                    consequence.evaluate(environment)
                } else {
                    alternative.evaluate(environment)
                }
            }
            Node::Sequence(ref head, ref more) => {
                head.evaluate(environment);
                more.evaluate(environment);
                Node::donothing()
            }
            Node::While(ref cond, ref body) => {
                if cond.evaluate(environment).condition() {
                    body.evaluate(environment);
                    self.evaluate(environment)
                } else {
                    Node::donothing()
                }
            }
        }
    }

    pub fn to_ruby(&self) -> String {
        match *self {
            Node::Number(v) => format!("-> e {{ {} }}", v),
            Node::Boolean(v) => format!("-> e {{ {} }}", v),
            Node::Variable(ref name) => format!("-> e {{ e[:{}] }}", name),
            Node::Add(ref l, ref r) => format!("-> e {{ ({}).call(e) + ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::Multiply(ref l, ref r) => format!("-> e {{ ({}).call(e) * ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::LessThan(ref l, ref r) => format!("-> e {{ ({}).call(e) < ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::Assign(ref name, ref expr) => format!("-> e {{ e.merge({{ :{} => ({}).call(e) }}) }}", name, expr.to_ruby()),
            Node::DoNothing => "-> e { e }".to_string(),
            Node::If(ref condition, ref consequence, ref alternative) => format!(
                "-> e {{if ({}).call(e) then ({}).call(e) else ({}).call(e) end }}",
                condition.to_ruby(), consequence.to_ruby(), alternative.to_ruby()
                ),
            Node::Sequence(ref first, ref second) => format!("-> e {{ ({}).call(({}).call(e)) }}", second.to_ruby(), first.to_ruby()),
            Node::While(ref condition, ref body) => format!("-> e {{ while ({}).call(e); e = ({}).call(e); end; e }}", condition.to_ruby(), body.to_ruby()),
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
