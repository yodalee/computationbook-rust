
use pdarule::{PDARule};
use pdaconfiguration::{PDAConfiguration};

#[derive(Clone)]
pub struct DPDARulebook {
    rules: Vec<PDARule>,
}

impl DPDARulebook {
    pub fn new(rules: &[PDARule]) -> Self {
        DPDARulebook{rules: rules.to_vec()}
    }

    pub fn next_config(&self, config: &PDAConfiguration, c: char) -> PDAConfiguration {
        match self.rule_for(config, c) {
            Some(rule)  => rule.follow(config),
            None => config.stuck(),
        }
    }

    pub fn rule_for(&self, config: &PDAConfiguration, c: char) -> Option<&PDARule> {
        self.rules.iter().find(|rule| rule.applies_to(config, c))
    }

    pub fn applies_to(&self, config: &PDAConfiguration, c: char) -> bool {
        self.rule_for(config, c).is_some()
    }

    pub fn follow_free_moves(&self, config: &PDAConfiguration) -> PDAConfiguration {
        if self.applies_to(config, '\0') {
            self.follow_free_moves(&self.next_config(config, '\0'))
        } else {
            config.clone()
        }
    }
}

