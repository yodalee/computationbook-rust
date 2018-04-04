use super::syntax::{Node};
use super::environment::{Environment};

pub trait Evaluate {
    fn evaluate(&self, environment: &mut Environment) -> Box<Node>;
}

impl Evaluate for Node {
    fn evaluate(&self, environment: &mut Environment) -> Box<Node> {
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
}
