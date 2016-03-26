
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
            None => panic!("No valid next config"),
        }
    }

    pub fn rule_for(&self, config: &PDAConfiguration, c: char) -> Option<&PDARule> {
        self.rules.iter().find(|rule| rule.applies_to(config, c))
    }
}

