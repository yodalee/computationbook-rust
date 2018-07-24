use std::collections::HashSet;
use std::hash::Hash;

use helper::{to_hashset};

use super::farule::{FARule};
use super::nfa::{NFA};
use super::nfarulebook::{NFARulebook};

pub struct NFADesign<T> {
    start_state: T,
    accept_states: HashSet<T>,
    rulebook: NFARulebook<T>
}

impl<T: Eq + Clone + Hash> NFADesign<T> {
    pub fn new(start_state: &T,
               accept_states: &HashSet<T>,
               rulebook: &NFARulebook<T>) -> Self {
        NFADesign{
            start_state: start_state.clone(),
            accept_states: accept_states.clone(),
            rulebook: rulebook.clone()
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut nfa = NFA::new(
            &to_hashset(&[self.start_state.clone()]),
            &self.accept_states,
            &self.rulebook);
        nfa.read_string(s);
        nfa.accepting()
    }

    pub fn start_state(&self) -> T { self.start_state.clone() }
    pub fn accept_state(&self) -> HashSet<T> { self.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule<T>> { self.rulebook.rules() }
}
