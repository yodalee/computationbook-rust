use std::collections::HashSet;

use nfarulebook::{NFARulebook};

#[derive(Clone)]
pub struct NFA {
    current_state: HashSet<u32>,
    accept_states: u32,
    rulebook: NFARulebook,
}

impl NFA {
    pub fn new(current_state: HashSet<u32>, accept_states: u32, rulebook: &NFARulebook) -> Self {
        NFA{current_state: current_state, accept_states: accept_states, rulebook: rulebook.clone()}
    }

    pub fn accepting(&self) -> bool {
        self.current_state.contains(&self.accept_states)
    }

    pub fn read_character(&mut self, character: char) {
        self.current_state = self.rulebook.next_states(&self.current_state, character);
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            self.read_character(c);
        }
    }
}
