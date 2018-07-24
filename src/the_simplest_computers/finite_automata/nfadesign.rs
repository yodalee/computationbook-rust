use std::collections::HashSet;
use std::hash::Hash;

use helper::{to_hashset};

use super::farule::{FARule};
use super::nfa::{NFA};
use super::nfarulebook::{NFARulebook};

#[derive(Clone)]
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

    pub fn to_nfa_with_state(&self, start_state: &HashSet<T>) -> NFA<T> {
        NFA::new(
            start_state,
            &self.accept_states,
            &self.rulebook)
    }

    pub fn to_nfa(&self) -> NFA<T> {
        self.to_nfa_with_state(&to_hashset(&[self.start_state.clone()]))
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut nfa = self.to_nfa();
        nfa.read_string(s);
        nfa.accepting()
    }

    pub fn start_state(&self) -> T { self.start_state.clone() }
    pub fn accept_state(&self) -> HashSet<T> { self.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule<T>> { self.rulebook.rules() }
}
