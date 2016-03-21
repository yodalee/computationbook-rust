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
}
