mod pdarule;
mod pdaconfiguration;
mod dpdarulebook;
mod dpda;
mod dpdadesign;
mod npdarulebook;
mod npda;
mod helper;

use pdarule::{PDARule};
use pdaconfiguration::{PDAConfiguration};
use dpdarulebook::{DPDARulebook};
use dpda::{DPDA};
use dpdadesign::{DPDADesign};
use npdarulebook::{NPDARulebook};
use npda::{NPDA};
use helper::{toHashSet};

pub fn main() {
    println!("************************************");
    println!("Demo Deterministic PushDown Automata");
    println!("************************************");
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

    //blow up
    //DPDARulebook::new(&[PDARule::new(1, '\0', 1, '$', &['$'])])
    //    .follow_free_moves(&PDAConfiguration::new(1, &['$']));

    dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
    dpda.read_string("(()(");
    println!("accept? {}", dpda.accept());
    println!("config: {}", dpda.current_config());
    dpda.read_string("))()");
    println!("accept? {}", dpda.accept());
    println!("config: {}", dpda.current_config());

    let mut dpda_design = DPDADesign::new(1, '$', &[1], &rulebook);
    println!("accept: '(((((((((())))))))))': {}", dpda_design.accept("(((((((((())))))))))"));
    println!("accept: '()(())((()))(()(()))': {}", dpda_design.accept("()(())((()))(()(()))"));
    println!("accept: '(()(()(()()(()()))()': {}", dpda_design.accept("(()(()(()()(()()))()"));

    //blow up println!("accept: '())': {}", dpda_design.accept("())"));

    dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
    dpda.read_string("())");
    println!("{}", dpda.config);
    println!("accept? {}", dpda.accept());
    println!("stuck? {}", dpda.is_stuck());
    println!("{}", dpda_design.accept("())"));

    rulebook = DPDARulebook::new(&[PDARule::new(1, 'a', 2, '$', &['a', '$']),
        PDARule::new(1, 'b', 2, '$', &['b', '$']), PDARule::new(2, 'a', 2, 'a', &['a', 'a']),
        PDARule::new(2, 'b', 2, 'b', &['b', 'b']), PDARule::new(2, 'a', 2, 'b', &[]),
        PDARule::new(2, 'b', 2, 'a', &[]), PDARule::new(2, '\0', 1, '$', &['$'])
    ]);
    dpda_design = DPDADesign::new(1, '$', &[1], &rulebook);
    println!("demo: contains equal a, b: accept ababab? {}, bbbaaaab? {}, baa? {}",
        dpda_design.accept("ababab"), dpda_design.accept("bbbaaaab"), dpda_design.accept("baa"));

    rulebook = DPDARulebook::new(&[PDARule::new(1, 'a', 1, '$', &['a', '$']),
        PDARule::new(1, 'a', 1, 'a', &['a', 'a']), PDARule::new(1, 'a', 1, 'b', &['a', 'b']),
        PDARule::new(1, 'b', 1, '$', &['b', '$']), PDARule::new(1, 'b', 1, 'a', &['b', 'a']),
        PDARule::new(1, 'b', 1, 'b', &['b', 'b']), PDARule::new(1, 'm', 2, '$', &['$']),
        PDARule::new(1, 'm', 2, 'a', &['a']), PDARule::new(1, 'm', 2, 'b', &['b']),
        PDARule::new(2, 'a', 2, 'a', &[]), PDARule::new(2, 'b', 2, 'b', &[]),
        PDARule::new(2, '\0', 3, '$', &['$'])
    ]);
    dpda_design = DPDADesign::new(1, '$', &[3], &rulebook);
    println!("demo: palindrome with m in middle");
    println!("accept abmba?: {}, accept babbamabbab?: {}, accept abmb?: {}, accept baambaa?: {}",
        dpda_design.accept("abmba"), dpda_design.accept("babbamabbab"), dpda_design.accept("abmb"), dpda_design.accept("baambaa"));

    println!("****************************************");
    println!("Demo Non-Deterministic PushDown Automata");
    println!("****************************************");

    let mut rulebook = NPDARulebook::new(&[
        PDARule::new(1, 'a', 1, '$', &['a', '$']),
        PDARule::new(1, 'a', 1, 'a', &['a', 'a']),
        PDARule::new(1, 'a', 1, 'b', &['a', 'b']),
        PDARule::new(1, 'b', 1, '$', &['b', '$']),
        PDARule::new(1, 'b', 1, 'a', &['b', 'a']),
        PDARule::new(1, 'b', 1, 'b', &['b', 'b']),
        PDARule::new(1, '\0', 2, '$', &['$']),
        PDARule::new(1, '\0', 2, 'a', &['a']),
        PDARule::new(1, '\0', 2, 'b', &['b']),
        PDARule::new(2, 'a', 2, 'a', &[]),
        PDARule::new(2, 'b', 2, 'b', &[]),
        PDARule::new(2, '\0', 3, '$', &['$'])
    ]);
    config = PDAConfiguration::new(1, &['$']);
    let mut npda = NPDA::new(&toHashSet(&[config]), &toHashSet(&[3]), &rulebook);
    println!("{}", npda.accept());
    for config in npda.current_config().iter() {
        println!("{}", config);
    }
    npda.read_string("abb");
    println!("{}", npda.accept());
    for config in npda.current_config().iter() {
        println!("{}", config);
    }
    npda.read_character('a');
    println!("{}", npda.accept());
    for config in npda.current_config().iter() {
        println!("{}", config);
    }
}

