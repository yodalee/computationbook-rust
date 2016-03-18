mod syntax;

use syntax::Node;

pub fn main() {
    let n = Node::number(3);
    print!("{}\n", n);

    let m = Node::multiply(Node::add(Node::number(1), Node::number(2)), Node::add(Node::number(3), Node::number(4)));
    print!("{}\n", m);
}
