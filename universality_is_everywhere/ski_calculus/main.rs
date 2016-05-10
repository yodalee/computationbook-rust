mod ski;
mod lambda;
mod lc_to_ski;

use ski::{SKI, SKICombinator};
use lambda::{Lambda};
use lc_to_ski::{toSKI};

pub fn main() {
    let x = SKI::skisymbol("x");
    println!("{}", x);

    let mut expr = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::skicall(SKI::i(), x.clone()));
    println!("{}", expr);

    let y = SKI::skisymbol("y");
    let z = SKI::skisymbol("z");

    expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::skisymbol("S"), x.clone()), y.clone()), z.clone());

    let combinator = expr.left().left().left();
    let first_arg = expr.left().left().right();
    let second_arg = expr.left().right();
    let third_arg = expr.right();
    println!("{}", expr);
    println!("{0} {1} {2} {3}", combinator, first_arg, second_arg, third_arg);
    println!("{}", combinator.call(vec![first_arg, second_arg, third_arg]));

    println!("{}", expr);
    println!("{}", expr.combinator());
    println!("{}", expr.combinator().call(expr.arguments()));

    expr = SKI::skicall(SKI::skicall(x.clone(), y.clone()), z.clone());
    println!("{0} is callable? {1}", expr, expr.callable(expr.arguments()));
    expr = SKI::skicall(SKI::skicall(SKI::skisymbol("S"), x.clone()), y.clone());
    println!("{0} is callable? {1}", expr, expr.callable(expr.arguments()));
    expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::skisymbol("S"), x.clone()), y.clone()), z.clone());
    println!("{0} is callable? {1}", expr, expr.combinator().callable(expr.arguments()));

    let swap = SKI::skicall(
        SKI::skicall(
            SKI::skisymbol("S"),
            SKI::skicall(
                SKI::skisymbol("K"),
                SKI::skicall(
                    SKI::skisymbol("S"),
                    SKI::skisymbol("I")
                ),
            ),
        ),
        SKI::skisymbol("K"),
    );
    expr = SKI::skicall(SKI::skicall(swap.clone(), SKI::skisymbol("x")), SKI::skisymbol("y"));

    println!("swap: {}", swap);
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("{}", expr);

    let mut original = SKI::skicall(SKI::skicall(SKI::skisymbol("S"), SKI::skisymbol("K")), SKI::skisymbol("I"));
    println!("{}", original);
    let mut function = original.as_function_of("x");
    println!("{}", function);
    println!("{}", function.reducible());
    expr = SKI::skicall(function.clone(), y.clone());
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("expr: {} == original: {}", expr, original);

    original = SKI::skicall(SKI::skicall(SKI::skisymbol("S"), SKI::skisymbol("x")), SKI::skisymbol("I"));
    println!("{}", original);
    function = original.as_function_of("x");
    println!("{}", function);
    expr = SKI::skicall(function.clone(), y.clone());
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("expr reduce: {} != original: {}", expr, original);

    let two = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x")))));
    println!("{} to SKI: {}", two, two.to_ski());
    let (inc, zero) = (SKI::skisymbol("inc"), SKI::skisymbol("zero"));

    expr = SKI::skicall(SKI::skicall(two.to_ski(), inc), zero);
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("{}", expr);

    let identity = SKI::skicall(SKI::skicall(SKI::skisymbol("S"), SKI::skisymbol("K")), SKI::skisymbol("K"));
    println!("{}", identity);
    expr = SKI::skicall(identity, SKI::skisymbol("x"));
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("{}", expr);
}
