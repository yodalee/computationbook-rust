mod ski;

use ski::{SKI};

pub fn main() {
    let mut x = SKI::skisymbol("x");
    println!("{}", x);

    let mut expr = SKI::skicall(SKI::skicall(SKI::skisymbol("S"), SKI::skisymbol("K")), SKI::skicall(SKI::skisymbol("I"), x.clone()));
    println!("{}", expr);

    let mut y = SKI::skisymbol("y");
    let mut z = SKI::skisymbol("z");

    expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::skisymbol("S"), x.clone()), y.clone()), z.clone());

    println!("{}", expr);
    let combinator = expr.left().left().left();
    let first_arg = expr.left().left().right();
    let second_arg = expr.left().right();
    let third_arg = expr.right();
    println!("{0}\n{1}\n{2}\n{3}", combinator, first_arg, second_arg, third_arg);
    println!("{}", combinator.call(vec![first_arg, second_arg, third_arg]));
}
