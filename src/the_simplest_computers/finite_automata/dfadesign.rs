use super::dfa::{DFA};
use super::dfarulebook::{DFARulebook};

pub struct DFADesign<T> {
    dfa: DFA<T>,
}

impl<T: Eq + Clone> DFADesign<T> {
    pub fn new(start_state: T, accept_states: Vec<T>, rulebook: &DFARulebook<T>) -> Self {
        DFADesign{ dfa: DFA::new(start_state, accept_states, &rulebook.clone())}
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_dfa = self.dfa.clone();
        to_dfa.read_string(s);
        to_dfa.accepting()
    }
}
