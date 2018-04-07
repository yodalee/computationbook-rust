use super::tm_rule::{TMRule};
use super::tm_configuration::{TMConfiguration};

pub struct DTMRulebook {
    pub rules: Vec<TMRule>,
}

impl DTMRulebook {
    pub fn new(rules: Vec<TMRule>) -> DTMRulebook {
        DTMRulebook {rules: rules}
    }

    pub fn next_configuration(&self, config: &mut TMConfiguration) {
        match self.rule_for(config) {
            Some(rule) => rule.follow(config),
            None => {},
        }
    }

    fn rule_for(&self, config: &TMConfiguration) -> Option<&TMRule> {
        self.rules.iter().find(|rule| rule.applies_to(config))
    }

    pub fn applies_to(&self, config: &TMConfiguration) -> bool{
        self.rule_for(config).is_some()
    }
}
