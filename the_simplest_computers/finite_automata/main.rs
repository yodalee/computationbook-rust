mod farule;
mod dfarulebook;
mod dfa;
mod dfadesign;

mod nfarulebook;
mod nfa;
mod nfadesign;
mod helper;

use farule::{FARule};
use dfarulebook::{DFARulebook};
use dfa::{DFA};
use dfadesign::{DFADesign};

use nfarulebook::{NFARulebook};
use nfa::{NFA};
use nfadesign::{NFADesign};

use helper::{to_hashset};

pub fn main() {
    println!("*****************");
    println!("DFA demonstration");
    println!("*****************");
    let rulebook = DFARulebook::new(
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

    let dfa_design = DFADesign::new(1, vec![3], &rulebook);
    println!("accept a: {}, baa: {}, baba: {}", dfa_design.accept("a"), dfa_design.accept("baa"), dfa_design.accept("baba"));

    println!("*****************");
    println!("NFA demonstration");
    println!("*****************");
    let rulebook = NFARulebook::new(
        vec![FARule::new(1, 'a', 1), FARule::new(1, 'b', 1), FARule::new(1, 'b', 2), FARule::new(2, 'a', 3),
             FARule::new(2, 'b', 3), FARule::new(3, 'a', 4), FARule::new(3, 'b', 4)]);
    println!("{:?}", rulebook.next_states(&to_hashset(&[1]), 'b'));
    println!("{:?}", rulebook.next_states(&to_hashset(&[1,2]), 'a'));
    println!("{:?}", rulebook.next_states(&to_hashset(&[1,3]), 'b'));

    println!("{}", NFA::new(
            &to_hashset(&[1]),
            &to_hashset(&[4]),
            &rulebook).accepting());
    println!("{}", NFA::new(
            &to_hashset(&[1,2,4]),
            &to_hashset(&[4]),
            &rulebook).accepting());

    let mut nfa = NFA::new(
        &to_hashset(&[1]),
        &to_hashset(&[4]), &rulebook);

    println!("{}", nfa.accepting());
    nfa.read_character('b');
    println!("{}", nfa.accepting());
    nfa.read_character('a');
    println!("{}", nfa.accepting());
    nfa.read_character('b');
    println!("{}", nfa.accepting());

    let mut nfa = NFA::new(
        &to_hashset(&[1]),
        &to_hashset(&[4]),
        &rulebook);
    println!("{}", nfa.accepting());
    nfa.read_string("bbbbb");
    println!("{}", nfa.accepting());

    let mut nfa_design = NFADesign::new(1, &to_hashset(&[4]), &rulebook);
    println!("accept bab: {}, bbbbb: {}, bbabb: {}",
             nfa_design.accept("bab"),
             nfa_design.accept("bbbbb"),
             nfa_design.accept("bbabb"));

    println!("********************************");
    println!("NFA with free move demonstration");
    println!("********************************");
    let rulebook = NFARulebook::new(
        vec![FARule::new(1, '\0', 2), FARule::new(1, '\0', 4), FARule::new(2, 'a', 3), FARule::new(3, 'a', 2),
             FARule::new(4, 'a', 5), FARule::new(5, 'a', 6), FARule::new(6, 'a', 4)]);
    println!("{:?}", rulebook.next_states(&to_hashset(&[1]), '\0'));
    println!("{:?}", rulebook.follow_free_moves(&to_hashset(&[1])));

    nfa_design = NFADesign::new(1, &to_hashset(&[2, 4]), &rulebook);
    println!("accept aa: {}, aaa: {}, aaaaa: {}, aaaaaa: {}",
             nfa_design.accept("aa"), nfa_design.accept("aaa"), nfa_design.accept("aaaaa"), nfa_design.accept("aaaaaa"));
}
