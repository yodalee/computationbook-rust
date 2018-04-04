use tm_configuration::{TMConfiguration};
use dtm_rulebook::{DTMRulebook};

use std::collections::HashSet;

pub struct DTM {
    pub current_configuration: TMConfiguration,
    accept_state: HashSet<i32>,
    rulebook: DTMRulebook,
}

impl DTM {
    pub fn new(config: TMConfiguration, accept_state: HashSet<i32>, rulebook: DTMRulebook) -> DTM {
        DTM {current_configuration: config, accept_state: accept_state.clone(),
            rulebook: rulebook}
    }

    pub fn accepting(&self) -> bool {
        self.accept_state.contains(&self.current_configuration.state)
    }

    pub fn step(&mut self) {
        self.rulebook.next_configuration(&mut self.current_configuration)
    }

    pub fn run(&mut self) {
        while !(self.accepting() || self.stuck()) {
            self.step()
        }
    }

    pub fn stuck(&self) -> bool {
        !self.accepting() && !self.rulebook.applies_to(
            &self.current_configuration)
    }
}
