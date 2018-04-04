use std::collections::{HashSet};
use pdaconfiguration::{PDAConfiguration};
use npdarulebook::{NPDARulebook};

pub struct NPDA {
    pub configs: HashSet<PDAConfiguration>,
    accept_states: HashSet<u32>,
    rulebook: NPDARulebook,
}

impl NPDA {
    pub fn new(configs: &HashSet<PDAConfiguration>, accept_state: &HashSet<u32>, rulebook: &NPDARulebook) -> Self {
        NPDA{
            configs: configs.clone(),
            accept_states: accept_state.clone(),
            rulebook: rulebook.clone()}
    }

    pub fn current_config(&self) -> HashSet<PDAConfiguration> {
        self.rulebook.follow_free_moves(&self.configs)
    }

    pub fn accept(&self) -> bool {
        self.current_config()
            .iter()
            .any(|config| config.state.is_some() && self.accept_states.contains(&config.state.unwrap()))
    }

    pub fn read_character(&mut self, c: char) {
        self.configs = self.rulebook.next_configs(&self.current_config(), c)
    }

    pub fn read_string(&mut self, s: &str) {
        for c in s.chars() {
            self.read_character(c)
        }
    }
}
