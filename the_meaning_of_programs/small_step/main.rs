mod machine;
mod syntax;
mod environment;

use machine::Machine;
use syntax::Node;
use environment::Environment;

pub fn main() {
    let n = Node::number(3);
    print!("{}\n", n);
    print!("{}\n", n.reducible());

    let m = Node::add(Node::multiply(Node::number(1), Node::number(2)), Node::multiply(Node::number(3), Node::number(4)));
    print!("{}\n", m);
    print!("{}\n", m.reducible());

    let mut machine = Machine::new_with_empty_env(m);
    machine.run();

    machine = Machine::new_with_empty_env(Node::lessthan(Node::number(5), Node::add(Node::number(2), Node::number(2))));
    machine.run();

    let mut env = Environment::new();
    env.add("x", Node::number(3));
    env.add("y", Node::number(4));

    machine = Machine::new(Node::add(Node::variable("x"), Node::variable("y")), env);
    machine.run();
}
