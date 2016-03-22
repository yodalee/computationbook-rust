use farule::FARule;

pub struct DFARulebook {
    rules: Vec<FARule>,
}

impl DFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        DFARulebook{rules: rules}
    }
    pub fn next_state(&self, state: u32, character: char) -> u32 {
        match self.rule_for(state, character) {
            Some(rule) => rule.follow(),
            None => panic!("No valid next state"),
        }
    }

    pub fn rule_for(&self, state: u32, character: char) -> Option<&FARule> {
        self.rules.iter().find(|x| x.applies_to(state, character))
    }
}
