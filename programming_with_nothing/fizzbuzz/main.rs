mod solution;
mod helper;

use solution::*;
use helper::*;

pub fn main() {
    println!("************************");
    println!("Demo Non-negative number");
    println!("{}", to_integer(ZERO));
    println!("{}", to_integer(ONE));
    println!("{}", to_integer(THREE));
    println!("{}", to_integer(FIVE));
    println!("{}", to_integer(FIFTEEN));
    println!("{}", to_integer(HUNDRED));

    println!("************");
    println!("Demo Boolean");
    println!("{}", to_boolean(TRUE));
    println!("{}", to_boolean(FALSE));
    println!("{}", IF(TRUE)("happy", "sad"));
    println!("{}", IF(FALSE)("happy", "sad"));

}
