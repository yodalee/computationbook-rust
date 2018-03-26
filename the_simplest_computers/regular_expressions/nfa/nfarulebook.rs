use std::collections::HashSet;
use std::rc::{Rc};

use nfa::farule::{FARule, State};

#[derive(Clone)]
pub struct NFARulebook {
    rules: Vec<FARule>,
}

impl NFARulebook {
    pub fn new(rules: Vec<FARule>) -> Self {
        NFARulebook{rules: rules}
    }
    pub fn next_states(&self, states: &HashSet<Rc<State>>, character: char) -> HashSet<Rc<State>> {
        let mut next_states: HashSet<Rc<State>> = HashSet::new();
        for state in states.iter() {
            for next_state in self.follow_rules_for(state, character) {
                next_states.insert(next_state);
            }
        }
        next_states
    }

    pub fn follow_rules_for(&self, state: &Rc<State>, character: char) -> Vec<Rc<State>> {
        self.rules.iter().filter(|rule| rule.applies_to(state, character)).map(|rule| rule.follow()).collect()
    }

    pub fn follow_free_moves(&self, states: &HashSet<Rc<State>>) -> HashSet<Rc<State>> {
        let more_states = self.next_states(states, '\0');
        if more_states.is_subset(states) {
            states.clone()
        } else {
            self.follow_free_moves(&states.union(&more_states).cloned().collect())
        }
    }

    pub fn rules(&self) -> Vec<FARule> { self.rules.clone() }
}
