use super::dfarulebook::{DFARulebook};

#[derive(Clone)]
pub struct DFA<T> {
    current_state: T,
    accept_states: Vec<T>,
    rulebook: DFARulebook<T>,
}

impl<T: Eq + Clone> DFA<T> {
    pub fn new(current_state: T,
               accept_states: Vec<T>,
               rulebook: &DFARulebook<T>) -> Self {
        DFA {
            current_state: current_state,
            accept_states: accept_states.clone(),
            rulebook: rulebook.clone()
        }
    }

    pub fn accepting(&self) -> bool {
        match self.accept_states.iter()
                                .find(|x| x == &&self.current_state) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn read_character(&mut self, character: char) {
        self.current_state = self.rulebook.next_state(&self.current_state, character);
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            self.read_character(c);
        }
    }
}
