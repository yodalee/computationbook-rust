use dfarulebook::{DFARulebook};

pub struct DFA {
    current_state: u32,
    accept_states: Vec<u32>,
    rulebook: DFARulebook,
}

impl DFA {
    pub fn new(current_state: u32, accept_states: Vec<u32>, rulebook: &DFARulebook) -> Self {
        DFA{current_state: current_state, accept_states: accept_states, rulebook: rulebook.clone()}
    }

    pub fn accepting(&self) -> bool {
        match self.accept_states.iter().find(|&&x| x == self.current_state) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn read_character(&mut self, character: char) {
        self.current_state = self.rulebook.next_state(self.current_state, character);
    }
}
