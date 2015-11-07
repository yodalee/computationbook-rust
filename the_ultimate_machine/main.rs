mod tape;
mod tm_configuration;
mod tm_rule;

use tape::{Tape};
use tm_configuration::{TMConfiguration};
use tm_rule::{TMRule, Direction};

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

    let rule = TMRule::new(1, '0', 2, '1', Direction::Right);
    println!("{}", rule.applies_to(TMConfiguration::new(1, Tape::new("", '0', "", '_'))));
    println!("{}", rule.applies_to(TMConfiguration::new(1, Tape::new("", '1', "", '_'))));
    println!("{}", rule.applies_to(TMConfiguration::new(2, Tape::new("", '0', "", '_'))));
}
