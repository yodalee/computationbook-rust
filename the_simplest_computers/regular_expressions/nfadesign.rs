use std::collections::HashSet;
use std::rc::Rc;

use farule::{FARule, State};
use nfa::{NFA};
use nfarulebook::{NFARulebook};
use helper::{toHashSet};

pub struct NFADesign {
    pub start_state: Rc<State>,
    nfa: NFA,
}

impl NFADesign {
    pub fn new(start_state: &Rc<State>, accept_states: &HashSet<Rc<State>>, rulebook: &NFARulebook) -> Self {
        NFADesign{
            start_state: start_state.clone(),
            nfa: NFA::new(
                 &toHashSet(&[start_state.clone()]),
                 &accept_states,
                 &rulebook)
        }
    }

    pub fn accept(&self, s: &str) -> bool {
        let mut to_nfa = self.nfa.clone();
        to_nfa.read_string(s);
        to_nfa.accepting()
    }

    pub fn start_state(&self) -> Rc<State> { self.start_state.clone() }
    pub fn accept_state(&self) -> HashSet<Rc<State>> { self.nfa.accept_states.clone() }
    pub fn rules(&self) -> Vec<FARule> { self.nfa.rulebook.rules() }
}
