use super::syntax::{Node};
use super::environment::{Environment};

pub trait Reduce {
    fn reducible(&self) -> bool;
    fn reduce(&self, environment: &mut Environment) -> Box<Node>;
}

impl Reduce for Node {
    fn reducible(&self) -> bool {
        match *self {
            Node::Number(_) | Node::Boolean(_) | Node::DoNothing => false,
            _ => true,
        }
    }

    fn reduce(&self, environment: &mut Environment) -> Box<Node> {
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
                    if condition.condition() {
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
            Node::While(ref cond, ref body) => {
                Node::if_cond_else(
                    cond.clone(),
                    Node::sequence(body.clone(), Box::new(self.clone())),
                    Node::donothing()
                )
            }
            _ => panic!("Non reducible type found: {}", *self)
        }
    }
}
