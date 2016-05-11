mod tag_rule;
mod cyclic_tag_rule;
mod cyclic_tag_rulebook;
mod cyclic_tag_system;

use cyclic_tag_rule::{CyclicTagRule};
use cyclic_tag_rulebook::{CyclicTagRulebook};
use cyclic_tag_system::{CyclicTagSystem};

pub fn main() {
    let rulebook = CyclicTagRulebook::new(&[CyclicTagRule::new("1"), CyclicTagRule::new("0010"), CyclicTagRule::new("10")].to_vec());

    let mut system = CyclicTagSystem::new("11", rulebook);

    for _ in 0..16 {
        println!("{}", system.current_string);
        system.step();
    }
    println!("{}", system.current_string);

    for _ in 0..20 {
        println!("{}", system.current_string);
        system.step();
    }
    println!("{}", system.current_string);
}
