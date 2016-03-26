mod pdaconfiguration;
mod pdarule;
mod dpdarulebook;
mod dpda;

use pdarule::{PDARule};
use pdaconfiguration::{PDAConfiguration};
use dpdarulebook::{DPDARulebook};
use dpda::{DPDA};

pub fn main() {
    let mut rule = PDARule::new(1, '(', 2, '$', &['b', '$']);
    println!("{}", rule);
    let mut config = PDAConfiguration::new(1, &['$']);
    println!("{}", config);
    println!("rule applies to? {}", rule.applies_to(&config, '('));
    println!("{}", rule.follow(&config));

    let mut rulebook = DPDARulebook::new(
        &[PDARule::new(1, '(', 2, '$', &['b', '$']), PDARule::new(2, '(', 2, 'b', &['b', 'b']),
        PDARule::new(2, ')', 2, 'b', &[]), PDARule::new(2, '\0', 1, '$', &['$'])]);
    config = rulebook.next_config(&config, '(');
    println!("{}", config);
    config = rulebook.next_config(&config, '(');
    println!("{}", config);
    config = rulebook.next_config(&config, ')');
    println!("{}", config);

    let mut dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
    println!("accept '' {}", dpda.accept());
    dpda.read_string("(()");
    println!("accept '(()' {}", dpda.accept());
    println!("config {}", dpda.config);

    config = PDAConfiguration::new(2, &['$']);
    println!("{}", config);
    println!("{}", rulebook.follow_free_moves(&config));

    dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
    dpda.read_string("(()(");
    println!("accept? {}", dpda.accept());
    println!("config: {}", dpda.current_config());
    dpda.read_string("))()");
    println!("accept? {}", dpda.accept());
    println!("config: {}", dpda.current_config());
}

