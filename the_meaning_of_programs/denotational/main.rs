mod syntax;
mod environment;

use syntax::Node;
use environment::Environment;

pub fn main() {
    println!("{}", Node::number(5).to_ruby());
    println!("{}", Node::boolean(false).to_ruby());
    let mut expr = Node::variable("x");
    println!("{}", expr.to_ruby());
    println!("{}", Node::add(Node::variable("x"), Node::number(1)).to_ruby());
    println!("{}", Node::lessthan(Node::add(Node::variable("x"), Node::number(1)), Node::number(3)).to_ruby());
    println!("{}", Node::assign("y", Node::add(Node::variable("x"), Node::number(1))).to_ruby());
    println!("{}", Node::while_node(
            Node::lessthan(Node::variable("x"), Node::number(5)),
            Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3)))).to_ruby());
}
