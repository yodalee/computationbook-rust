use std::collections::{HashSet};

use pdarule::{PDARule};
use pdaconfiguration::{PDAConfiguration};

#[derive(Clone)]
pub struct NPDARulebook {
    rules: Vec<PDARule>,
}

impl NPDARulebook {
    pub fn new(rules: &[PDARule]) -> Self {
        NPDARulebook{rules: rules.to_vec()}
    }

    pub fn next_configs(&self, configs: &HashSet<PDAConfiguration>, c: char) -> HashSet<PDAConfiguration> {
        let mut next_configs: HashSet<PDAConfiguration> = HashSet::new();
        for config in configs {
            for next_config in self.follow_rules_for(config, c) {
                next_configs.insert(next_config);
            }
        }
        next_configs
    }

    pub fn follow_rules_for(&self, config: &PDAConfiguration, c: char) -> Vec<PDAConfiguration> {
        self.rules.iter().filter(|rule| rule.applies_to(config, c)).map(|rule| rule.follow(config)).collect()
    }

    pub fn follow_free_moves(&self, configs: &HashSet<PDAConfiguration>) -> HashSet<PDAConfiguration> {
        let more_configs = self.next_configs(configs, '\0');
        if more_configs.is_subset(configs) {
            configs.clone()
        } else {
            self.follow_free_moves(&configs.union(&more_configs).cloned().collect())
        }
    }
}

