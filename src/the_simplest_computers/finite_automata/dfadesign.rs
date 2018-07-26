use super::dfa::{DFA};
use super::dfarulebook::{DFARulebook};

pub struct DFADesign<T> {
    start_state: T,
    accept_states: Vec<T>,
    rulebook: DFARulebook<T>,
}

impl<T: Eq + Clone> DFADesign<T> {
    pub fn new(start_state: T, accept_states: &Vec<T>, rulebook: &DFARulebook<T>) -> Self {
        DFADesign {
            start_state: start_state,
            accept_states: accept_states.clone(),
            rulebook: rulebook.clone()}
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut dfa = DFA::new(
            self.start_state.clone(), &self.accept_states, &self.rulebook);
        dfa.read_string(s);
        dfa.accepting()
    }
}
