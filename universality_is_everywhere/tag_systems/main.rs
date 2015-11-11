mod tag_rulebook;
mod tag_system;
mod tag_rule;

use tag_rulebook::{TagRulebook};
use tag_system::{TagSystem};
use tag_rule::{TagRule};

fn main() {
    println!("A tag system that double input forever");
    let rulebook = TagRulebook::new(2, vec!(
            TagRule::new('a', "aa"),
            TagRule::new('b', "bbbb")));
    let mut system = TagSystem::new("aabbbbbb", rulebook);
    for i in 0..4 {
        println!("{}", system.current_string);
        system.step()
    }
    println!("{}", system.current_string);

    println!("A tag system that double input, and stoppable");
    let rulebook = TagRulebook::new(2, vec!(
            TagRule::new('a', "cc"),
            TagRule::new('b', "dddd")));
    let mut system = TagSystem::new("aabbbbbb", rulebook);
    system.run();

    println!("A tag system that halves input");
    let rulebook = TagRulebook::new(2, vec!(
            TagRule::new('a', "cc"),
            TagRule::new('b', "d")));
    let mut system = TagSystem::new("aabbbbbbbbbbbb", rulebook);
    system.run();

    println!("A tag system that increment input");
    let rulebook = TagRulebook::new(2, vec!(
            TagRule::new('a', "ccdd"),
            TagRule::new('b', "dd")));
    let mut system = TagSystem::new("aabbbb", rulebook);
    system.run();

    println!("A tag system that double and increment input");
    let rulebook = TagRulebook::new(2, vec!(
        // double
        TagRule::new('a', "cc"),
        TagRule::new('b', "dddd"),
        // increment
        TagRule::new('c', "eeff"),
        TagRule::new('d', "ff")));
    let mut system = TagSystem::new("aabbbb", rulebook);
    system.run();

    println!("A tag system that tests input is odd or even");
    let rulebook = TagRulebook::new(2, vec!(
        TagRule::new('a', "cc"),
        TagRule::new('b', "d"),
        TagRule::new('c', "eo"),
        TagRule::new('d', ""),
        TagRule::new('e', "e")));
    let mut system = TagSystem::new("aabbbbbbbb", rulebook);
    system.run();

    println!("A tag system that tests input is odd or even");
    let rulebook = TagRulebook::new(2, vec!(
        TagRule::new('a', "cc"),
        TagRule::new('b', "d"),
        TagRule::new('c', "eo"),
        TagRule::new('d', ""),
        TagRule::new('e', "e")));
    let mut system = TagSystem::new("aabbbbbbbbbb", rulebook);
    system.run();
}

