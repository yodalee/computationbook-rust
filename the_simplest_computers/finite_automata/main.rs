mod farule;
mod dfarulebook;
mod dfa;
mod dfadesign;

mod nfarulebook;

use farule::{FARule};
use dfarulebook::{DFARulebook};
use dfa::{DFA};
use dfadesign::{DFADesign};

use nfarulebook::{NFARulebook};

pub fn main() {
    println!("*****************");
    println!("DFA demonstration");
    println!("*****************");
    let mut rulebook = DFARulebook::new(
        vec![FARule::new(1, 'a', 2), FARule::new(1, 'b', 1),
         FARule::new(2, 'a', 2), FARule::new(2, 'b', 3),
         FARule::new(3, 'a', 3), FARule::new(3, 'b', 3)
        ]);
    println!("{}", rulebook.next_state(1, 'a'));
    println!("{}", rulebook.next_state(1, 'b'));
    println!("{}", rulebook.next_state(2, 'b'));

    println!("{}", DFA::new(1, vec![1, 3], &rulebook).accepting());
    println!("{}", DFA::new(1, vec![3], &rulebook).accepting());

    let mut dfa = DFA::new(1, vec![3], &rulebook);
    println!("{}", dfa.accepting());
    dfa.read_character('b');
    println!("{}", dfa.accepting());
    dfa.read_character('b');
    for _ in 0..3 {
        dfa.read_character('a')
    }
    println!("{}", dfa.accepting());
    dfa.read_character('b');
    println!("{}", dfa.accepting());

    dfa = DFA::new(1, vec![3], &rulebook);
    println!("{}", dfa.accepting());
    dfa.read_string("baaab");
    println!("{}", dfa.accepting());

    let mut dfa_design = DFADesign::new(1, vec![3], &rulebook);
    println!("accept a: {}, baa: {}, baba: {}", dfa_design.accept("a"), dfa_design.accept("baa"), dfa_design.accept("baba"));

    println!("*****************");
    println!("NFA demonstration");
    println!("*****************");
    let rulebook = NFARulebook::new(
        vec![FARule::new(1, 'a', 1), FARule::new(1, 'b', 1), FARule::new(1, 'b', 2), FARule::new(2, 'a', 3),
             FARule::new(2, 'b', 3), FARule::new(3, 'a', 4), FARule::new(3, 'b', 4)]);
    println!("{:?}", rulebook.next_states(vec![1], 'b'));
    println!("{:?}", rulebook.next_states(vec![1,2], 'a'));
    println!("{:?}", rulebook.next_states(vec![1,3], 'b'));
}
