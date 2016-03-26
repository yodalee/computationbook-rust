use pdaconfiguration::{PDAConfiguration};
use dpdarulebook::{DPDARulebook};

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
        self.accept_states.iter().find(|&&x| x == self.current_config().state).is_some()
    }

    pub fn read_character(&mut self, c: char) {
        self.config = self.rulebook.next_config(&self.current_config(), c)
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            self.read_character(c)
        }
    }
}
