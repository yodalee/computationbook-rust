use std::collections::HashSet;
use std::rc::Rc;

use super::farule::{State};
use super::nfarulebook::{NFARulebook};

#[derive(Clone)]
pub struct NFA {
    current_state: HashSet<Rc<State>>,
    pub accept_states: HashSet<Rc<State>>,
    pub rulebook: NFARulebook,
}

impl NFA {
    pub fn new(current_state: &HashSet<Rc<State>>, accept_states: &HashSet<Rc<State>>, rulebook: &NFARulebook) -> Self {
        NFA{
            current_state: current_state.clone(),
            accept_states: accept_states.clone(),
            rulebook: rulebook.clone()}
    }

    pub fn current_state(&self) -> HashSet<Rc<State>> {
        self.rulebook.follow_free_moves(&self.current_state)
    }

    pub fn accepting(&self) -> bool {
        !self.current_state().is_disjoint(&self.accept_states)
    }

    pub fn read_character(&mut self, character: char) {
        self.current_state = self.rulebook.next_states(&self.current_state(), character);
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            self.read_character(c);
        }
    }
}
