use farule::FARule;

#[derive(Clone)]
pub struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook{rules: rules}
    }
    pub fn next_states(&self, states: Vec<u32>, character: char) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for state in states.iter() {
            v.append(&mut self.follow_rules_for(*state, character))
        }
        v.sort();
        v.dedup();
        v
    }

    pub fn follow_rules_for(&self, state: u32, character: char) -> Vec<u32> {
        self.rules.iter().filter(|rule| rule.applies_to(state, character)).map(|rule| rule.follow()).collect()
    }
}
