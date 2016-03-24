mod solution;
mod helper;

use solution::*;
use helper::*;

pub fn main() {
    println!("{}", to_integer(ZERO));
    println!("{}", to_integer(ONE));
    println!("{}", to_integer(THREE));
    println!("{}", to_integer(FIVE));
    println!("{}", to_integer(FIFTEEN));
    println!("{}", to_integer(HUNDRED));
}
