use std::collections::HashSet;
use std::hash::Hash;

use farule::{FARule};
use nfa::{NFA};
use nfarulebook::{NFARulebook};
use helper::{to_hashset};

pub struct NFADesign<T> {
    start_state: T,
    nfa: NFA<T>,
}

impl<T: Eq + Clone + Hash> NFADesign<T> {
    pub fn new(start_state: T,
               accept_states: &HashSet<T>,
               rulebook: &NFARulebook<T>) -> Self {
        NFADesign{
            start_state: start_state.clone(),
            nfa: NFA::new(
                 &to_hashset(&[start_state.clone()]),
                 &accept_states,
                 &rulebook)
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_nfa = self.nfa.clone();
        to_nfa.read_string(s);
        to_nfa.accepting()
    }

    pub fn start_state(&self) -> T { self.start_state.clone() }
    pub fn accept_state(&self) -> HashSet<T> { self.nfa.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule<T>> { self.nfa.rulebook.rules() }
}
