use dfa::{DFA};
use dfarulebook::{DFARulebook};

pub struct DFADesign {
    dfa: DFA,
}

impl DFADesign {
    pub fn new(start_state: u32, accept_states: Vec<u32>, rulebook: &DFARulebook) -> Self {
        DFADesign{ dfa: DFA::new(start_state, accept_states, &rulebook.clone())}
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_dfa = self.dfa.clone();
        to_dfa.read_string(s);
        to_dfa.accepting()
    }
}
