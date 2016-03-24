use std::collections::HashSet;

use farule::{FARule};
use nfa::{NFA};
use nfarulebook::{NFARulebook};
use helper::{toHashSet};

// start_state, size is preserved for regex
pub struct NFADesign {
    start_state: u32,
    pub size: u32,
    nfa: NFA,
}

impl NFADesign {
    pub fn new(start_state: u32, accept_states: &HashSet<u32>, rulebook: &NFARulebook) -> Self {
        let mut maxstate:u32 = 0;
        for r in rulebook.rules().iter() {
            if r.state > maxstate {
                maxstate = r.state
            }
            if r.next_state > maxstate {
                maxstate = r.next_state
            }
        }
        NFADesign{
            start_state: start_state,
            size: maxstate - start_state,
            nfa: NFA::new(
                 &toHashSet(&[start_state]),
                 &accept_states,
                 &rulebook)
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_nfa = self.nfa.clone();
        to_nfa.read_string(s);
        to_nfa.accepting()
    }

    pub fn start_state(&self) -> u32 { self.start_state }
    pub fn accept_state(&self) -> HashSet<u32> { self.nfa.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule> { self.nfa.rulebook.rules() }
}
