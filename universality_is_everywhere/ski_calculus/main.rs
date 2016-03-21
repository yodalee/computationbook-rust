mod ski;

use ski::{SKI};

pub fn main() {
    let mut x = SKI::skisymbol("x");
    println!("{}", x);

    let mut expr = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::skicall(SKI::i(), x));
    println!("{}", expr);
}
