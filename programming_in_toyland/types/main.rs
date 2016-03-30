mod syntaxtype;
mod syntax;
mod environment;
mod context;
mod evaluate;

use syntax::{Node};
use syntaxtype::{SyntaxType, Type};
use evaluate::{Evaluate};
use environment::{Environment};
use context::{Context};

pub fn main() {
    let mut context = Context::new();
    println!("{}", Node::add(Node::number(1), Node::number(2)).evaluate(&mut Environment::new()));
    //blow up println!("{}", Node::add(Node::number(1), Node::boolean(true)).evaluate(&mut Environment::new()));
    println!("{:?}", Node::add(Node::number(1), Node::number(2)).getType(&mut context));
    println!("{:?}", Node::add(Node::number(1), Node::boolean(true)).getType(&mut context));
    println!("{:?}", Node::lessthan(Node::number(1), Node::number(2)).getType(&mut context));
    println!("{:?}", Node::lessthan(Node::number(1), Node::boolean(true)).getType(&mut context));
    let expr = Node::add(Node::variable("x"), Node::variable("y"));
    println!("{:?}", expr.getType(&mut context));
    context.add("x", Type::Number);
    context.add("y", Type::Number);
    println!("{:?}", expr.getType(&mut context));
    context.add("y", Type::Boolean);
    println!("{:?}", expr.getType(&mut context));
    context = Context::new();
    println!("{:?}", Node::If(Node::lessthan(Node::number(1), Node::number(2)), Node::donothing(), Node::donothing()).getType(&mut context));
    println!("{:?}", Node::If(Node::add(Node::number(1), Node::number(2)), Node::donothing(), Node::donothing()).getType(&mut context));
    context.add("x", Type::Boolean);
    println!("{:?}", Node::while_node(Node::variable("x"), Node::donothing()).getType(&mut context));
    context.add("x", Type::Number);
    println!("{:?}", Node::while_node(Node::variable("x"), Node::donothing()).getType(&mut context));
    let mut stat = Node::while_node(
        Node::lessthan(Node::variable("x"), Node::number(5)),
        Node::assign("x", Node::add(Node::variable("x"), Node::number(3))));
    println!("{}", stat);
    context = Context::new();
    println!("{:?}", stat.getType(&mut context));
    context.add("x", Type::Number);
    println!("{:?}", stat.getType(&mut context));
    context.add("x", Type::Boolean);
    println!("{:?}", stat.getType(&mut context));

    stat = Node::sequence(Node::assign("x", Node::number(0)),
        Node::while_node(Node::boolean(true), Node::assign("x", Node::add(Node::variable("x"), Node::number(1)))));
    context.add("x", Type::Number);
    println!("{}", stat);
    println!("{:?}", stat.getType(&mut context));
    //blow up println!("{}", stat.evaluate(&mut Environment::new()));
    context.add("x", Type::Boolean);
    println!("{:?}", Node::sequence(stat, Node::assign("x", Node::boolean(true))).getType(&mut context));

    stat = Node::sequence(
        Node::if_cond_else(Node::variable("b"), Node::assign("x", Node::number(6)), Node::assign("x", Node::boolean(true))),
        Node::sequence(
            Node::if_cond_else(Node::variable("b"), Node::assign("y", Node::variable("x")), Node::assign("y", Node::number(1))),
            Node::assign("z", Node::add(Node::variable("y"), Node::number(1)))));
    println!("{}", stat);
    let mut environment = Environment::new();
    environment.add("b", Node::boolean(true));
    stat.evaluate(&mut environment);
    println!("{}", environment);
    environment = Environment::new();
    environment.add("b", Node::boolean(false));
    stat.evaluate(&mut environment);
    println!("{}", environment);
    context = Context::new();
    context.add("b", Type::Boolean);
    context.add("y", Type::Number);
    context.add("z", Type::Number);
    println!("{:?}", stat.getType(&mut context));
    context.add("x", Type::Number);
    println!("{:?}", stat.getType(&mut context));
    context.add("x", Type::Boolean);
    println!("{:?}", stat.getType(&mut context));

    context = Context::new(); context.add("x", Type::Number);
    environment = Environment::new();
    stat = Node::assign("x", Node::add(Node::variable("x"), Node::number(1)));
    stat.getType(&mut context);
    //blow up stat.evaluate(&mut environment);
}
