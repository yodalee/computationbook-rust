mod machine;
mod syntax;

use machine::Machine;
use syntax::Node;

pub fn main() {
    let n = Node::number(3);
    print!("{}\n", n);
    print!("{}\n", n.reducible());

    let m = Node::add(Node::multiply(Node::number(1), Node::number(2)), Node::multiply(Node::number(3), Node::number(4)));
    print!("{}\n", m);
    print!("{}\n", m.reducible());

    let mut machine = Machine::new(m);
    machine.run();

    machine = Machine::new(Node::lessthan(Node::number(5), Node::add(Node::number(2), Node::number(2))));
    machine.run();
}
