mod tape;
mod tm_configuration;
mod tm_rule;
mod dtm_rulebook;

use std::vec;
use tape::{Tape};
use tm_configuration::{TMConfiguration};
use tm_rule::{TMRule, Direction};
use dtm_rulebook::{DTMRulebook};

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
    println!("{}", rule.applies_to(&TMConfiguration::new(1, Tape::new("", '0', "", '_'))));
    println!("{}", rule.applies_to(&TMConfiguration::new(1, Tape::new("", '1', "", '_'))));
    println!("{}", rule.applies_to(&TMConfiguration::new(2, Tape::new("", '0', "", '_'))));

    let mut config = TMConfiguration::new(1, Tape::new("", '0', "", '_'));
    rule.follow(&mut config);
    println!("{}", config);

    let mut tape = Tape::new("101", '1', "", '_');
    let rulebook = DTMRulebook::new(vec!(
        TMRule::new(1, '0', 2, '1', Direction::Right),
        TMRule::new(1, '1', 1, '0', Direction::Left),
        TMRule::new(1, '_', 2, '1', Direction::Right),
        TMRule::new(2, '0', 2, '0', Direction::Right),
        TMRule::new(2, '1', 2, '1', Direction::Right),
        TMRule::new(2, '_', 3, '_', Direction::Left)
    ));
    let mut config = TMConfiguration::new(1, tape);
    println!("{}", config);
    rulebook.next_configuration(&mut config);
    println!("{}", config);
    rulebook.next_configuration(&mut config);
    println!("{}", config);
    rulebook.next_configuration(&mut config);
    println!("{}", config);
}
