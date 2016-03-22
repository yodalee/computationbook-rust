use std::collections::HashSet;

use farule::FARule;

#[derive(Clone)]
pub struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook{rules: rules}
    }
    pub fn next_states(&self, states: &HashSet<u32>, character: char) -> HashSet<u32> {
        let mut next_states: HashSet<u32> = HashSet::new();
        for state in states.iter() {
            for next_state in self.follow_rules_for(*state, character) {
                next_states.insert(next_state);
            }
        }
        next_states
    }

    pub fn follow_rules_for(&self, state: u32, character: char) -> Vec<u32> {
        self.rules.iter().filter(|rule| rule.applies_to(state, character)).map(|rule| rule.follow()).collect()
    }
}
