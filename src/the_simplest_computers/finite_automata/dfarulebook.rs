use super::farule::FARule;

#[derive(Clone)]
pub struct DFARulebook<T> {
    rules: Vec<FARule<T>>,
}

impl<T: Eq + Clone> DFARulebook<T> {
    pub fn new(rules: Vec<FARule<T>>) -> Self {
        DFARulebook{rules: rules}
    }

    pub fn next_state(&self, state: &T, character: char) -> T {
        match self.rule_for(state, character) {
            Some(rule) => rule.follow(),
            None => panic!("No valid next state"),
        }
    }

    pub fn rule_for(&self, state: &T, character: char) -> Option<&FARule<T>> {
        self.rules.iter().find(|x| x.applies_to(state, character))
    }
}
