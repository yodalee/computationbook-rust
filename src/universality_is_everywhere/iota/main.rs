mod ski;
mod skicombinator;
mod lambda;
mod lc_to_ski;

use ski::{SKI};
use skicombinator::{SKICombinator};
use lambda::{Lambda};
use lc_to_ski::{toSKI};

pub fn main() {
    let mut expr = SKI::s().to_iota();

    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);

    expr = SKI::k().to_iota();

    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);

    expr = SKI::i().to_iota();

    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);

    let mut identity = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::skicall(SKI::k(), SKI::k()));

    expr = SKI::skicall(identity, SKI::skisymbol("x"));
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce()
    }
    println!("{}", expr);

    let two = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x")))));
    let (inc, zero) = (SKI::skisymbol("inc"), SKI::skisymbol("zero"));
    println!("lambda two: {}", two);
    println!("ski two: {}", two.to_ski());
    println!("iota two: {}", two.to_ski().to_iota());

    expr = SKI::skicall(SKI::skicall(two.to_ski().to_iota(), inc), zero);
    while expr.reducible() {
        expr = expr.reduce()
    }
    println!("{}", expr)
}
