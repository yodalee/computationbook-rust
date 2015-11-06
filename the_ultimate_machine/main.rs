mod tape;

use tape::{Tape};

pub fn main() {
    //test Tape
    let mut tape = Tape::new("101", '1', "", '_');

    println!("{}", tape);
    println!("tape middle: {}", tape.middle);
    tape.move_head_left();
    tape.write('0');
    tape.move_head_right();
    tape.move_head_right();
    tape.write('0');
    println!("{}", tape);
}
