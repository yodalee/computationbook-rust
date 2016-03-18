mod machine;
mod syntax;

use machine::Machine;
use syntax::Node;

pub fn main() {
    let n = Node::number(3);
    print!("{}\n", n);
    print!("{}\n", n.reducible());

    let mut m = Node::multiply(Node::add(Node::number(1), Node::number(2)), Node::add(Node::number(3), Node::number(4)));
    print!("{}\n", m);
    print!("{}\n", m.reducible());

    let mut machine = Machine::new(m);
    machine.run();
}
