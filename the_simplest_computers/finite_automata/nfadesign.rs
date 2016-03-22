use std::collections::HashSet;

use nfa::{NFA};
use nfarulebook::{NFARulebook};

pub struct NFADesign {
    nfa: NFA,
}

impl NFADesign {
    pub fn new(start_state: u32, accept_states: &HashSet<u32>, rulebook: &NFARulebook) -> Self {
        NFADesign{
            nfa: NFA::new(
                 &[start_state].into_iter().cloned().collect::<HashSet<u32>>(),
                 &accept_states,
                 &rulebook)
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_nfa = self.nfa.clone();
        to_nfa.read_string(s);
        to_nfa.accepting()
    }
}
