use super::pdaconfiguration::{PDAConfiguration};
use super::dpdarulebook::{DPDARulebook};

pub struct DPDA {
    pub config: PDAConfiguration,
    accept_states: Vec<u32>,
    rulebook: DPDARulebook,
}

impl DPDA {
    pub fn new(config: &PDAConfiguration, accept_state: &[u32], rulebook: &DPDARulebook) -> Self {
        DPDA{config: config.clone(), accept_states: accept_state.to_vec(), rulebook: rulebook.clone()}
    }

    pub fn current_config(&self) -> PDAConfiguration {
        self.rulebook.follow_free_moves(&self.config)
    }

    pub fn accept(&self) -> bool {
        match self.current_config().state {
            Some(state) => self.accept_states.iter().find(|&&x| x == state).is_some(),
            None => false,
        }
    }

    pub fn is_stuck(&self) -> bool {
        self.config.is_stuck()
    }

    pub fn read_character(&mut self, c: char) {
        self.config = self.rulebook.next_config(&self.current_config(), c)
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            if !self.config.is_stuck() {
                self.read_character(c)
            }
        }
    }
}
