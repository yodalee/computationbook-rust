mod machine;
mod syntax;
mod environment;
mod reduce;
mod evaluate;
mod denotational;

use machine::Machine;
use syntax::Node;
use environment::Environment;
use reduce::Reduce;
use evaluate::Evaluate;
use denotational::Denotational;

pub fn main() {
    println!("************************");
    println!("Small step demonstration");
    println!("************************");
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

    env = Environment::new();
    env.add("x", Node::number(1));
    machine = Machine::new(
        Node::while_node(
            Node::lessthan(Node::variable("x"), Node::number(5)),
            Node::assign("x", Node::multiply(Node::variable("x"), Node::number(3)))
        ), env
    );

    machine.run();
    println!("{}", machine.environment);
    println!("**********************");
    println!("Big step demonstration");
    println!("**********************");
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

    println!("*********************");
    println!("to_ruby demonstration");
    println!("*********************");

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
