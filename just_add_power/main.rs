mod pdaconfiguration;
mod pdarule;

use pdarule::{PDARule};
use pdaconfiguration::{PDAConfiguration};

pub fn main() {
    let mut rule = PDARule::new(1, '(', 2, '$', &['b', '$']);
    println!("{}", rule);
    let mut config = PDAConfiguration::new(1, &['$']);
    println!("{}", config);
    println!("rule applies to? {}", rule.applies_to(&config, '('));
    println!("{}", rule.follow(&config));
}
