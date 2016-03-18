mod machine;
mod syntax;
mod environment;

use machine::Machine;
use syntax::Node;
use environment::Environment;

pub fn main() {
    let n = Node::number(3);
    println!("{}", n);
    println!("{}", n.reducible());

    let m = Node::add(Node::multiply(Node::number(1), Node::number(2)), Node::multiply(Node::number(3), Node::number(4)));
    println!("{}", m);
    println!("{}", m.reducible());

    let mut machine = Machine::new_with_empty_env(m);
    machine.run();

    machine = Machine::new_with_empty_env(Node::lessthan(Node::number(5), Node::add(Node::number(2), Node::number(2))));
    machine.run();

    let mut env = Environment::new();
    env.add("x", Node::number(3));
    env.add("y", Node::number(4));

    machine = Machine::new(Node::add(Node::variable("x"), Node::variable("y")), env);
    machine.run();

    let mut statement = Node::assign("x", Node::add(Node::variable("x"), Node::number(1)));
    let mut env = Environment::new();
    env.add("x", Node::number(2));

    println!("{}", statement.reducible());
    statement = statement.reduce(&mut env);
    println!("{0}; {1}", statement, env);
    statement = statement.reduce(&mut env);
    println!("{0}; {1}", statement, env);
    statement = statement.reduce(&mut env);
    println!("{0}; {1}", statement, env);
    println!("{}", statement.reducible());

    env = Environment::new();
    env.add("x", Node::boolean(true));

    machine = Machine::new(
        Node::if_cond_else(
            Node::variable("x"),
            Node::assign("y", Node::number(1)),
            Node::assign("y", Node::number(2))
        ), env
    );
    machine.run();
    println!("{}", machine.environment);

    env = Environment::new();
    env.add("x", Node::boolean(false));
    machine = Machine::new(
        Node::if_cond_else(
            Node::variable("x"),
            Node::assign("y", Node::number(1)),
            Node::donothing()
        ), env
    );
    machine.run();
    println!("{}", machine.environment);

    machine = Machine::new_with_empty_env(
        Node::sequence(
            Node::assign("x", Node::add(Node::number(1), Node::number(1))),
            Node::assign("y", Node::add(Node::variable("x"), Node::number(3))),
        )
    );
    machine.run();
    println!("{}", machine.environment);
}
