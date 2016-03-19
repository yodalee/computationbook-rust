mod syntax;
mod environment;

use syntax::Node;
use environment::Environment;

pub fn main() {
    let mut env = Environment::new();

    let mut n = Node::number(3);
    println!("{}", n.evaluate(&mut env));

    env.add("x", Node::number(23));
    n = Node::variable("x");
    println!("{}", n.evaluate(&mut env));

    env.add("x", Node::number(2));
    env.add("y", Node::number(5));
    n = Node::lessthan(Node::add(Node::variable("x"), Node::number(2)), Node::variable("y"));
    println!("{}", n.evaluate(&mut env));

    env = Environment::new();
    let mut statement = Node::sequence(
        Node::assign("x", Node::add(Node::number(1), Node::number(1))),
        Node::assign("y", Node::add(Node::variable("x"), Node::number(3)))
    );
    println!("{}", statement.evaluate(&mut env));
    println!("{}", env);

    env = Environment::new();
    statement = Node::while_node(
        Node::lessthan(Node::variable("x"), Node::number(5)),
        Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3))),
    );
    env.add("x", Node::number(1));
    println!("{}", statement.evaluate(&mut env));
    println!("{}", env);
}
