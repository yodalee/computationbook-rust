use std::collections::HashSet;
use std::rc::Rc;

use farule::{FARule, State};
use nfa::{NFA};
use nfarulebook::{NFARulebook};
use helper::{toHashSet};

pub struct NFADesign {
    nfa: NFA,
}

impl NFADesign {
    pub fn new(start_state: Rc<State>, accept_states: &HashSet<Rc<State>>, rulebook: &NFARulebook) -> Self {
        NFADesign{
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

    pub fn accept_state(&self) -> HashSet<Rc<State>> { self.nfa.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule> { self.nfa.rulebook.rules() }
}
