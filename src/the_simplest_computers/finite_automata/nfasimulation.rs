use std::collections::HashSet;
use std::hash::{Hash};
use std::fmt::Debug;

use super::farule::{FARule};
use super::nfadesign::{NFADesign};
use super::stateset::{StateSet};

pub struct NFASimulation<T> {
    nfa_design: NFADesign<T>
}

impl<T: Debug + Eq + Clone + Hash + Ord> NFASimulation<T> {
    pub fn new(nfa_design: &NFADesign<T>) -> NFASimulation<T> {
        NFASimulation {
            nfa_design: nfa_design.clone()
        }
    }

    pub fn next_state (&self, states: &HashSet<T>, c: char) -> HashSet<T> {
        let mut nfa = self.nfa_design.to_nfa_with_state(states);
        nfa.read_character(c);
        nfa.current_state()
    }

    pub fn rule_for(&self, states: &HashSet<T>) -> Vec<FARule<StateSet<T>>> {
        let start_set = StateSet::new(states);
        let alphabet = self.nfa_design.rulebook().alphabet();
        alphabet.iter()
                .map(|&c| FARule::new(&start_set, c, &StateSet::new(&self.next_state(states, c))))
                .collect()
    }

    pub fn discover_states_and_rules(&self, mut states: &mut HashSet<StateSet<T>>)
            -> (HashSet<StateSet<T>>, Vec<FARule<StateSet<T>>>) {
        let rules: Vec<_>  = states.iter()
                             .flat_map(|state| self.rule_for(&state.0))
                             .collect::<Vec<_>>();
        let more_states: HashSet<StateSet<T>> = rules.iter()
                                  .map(|rule| rule.follow())
                                  .collect();
        if more_states.is_subset(states) {
            (states.clone(), rules)
        } else {
            for state in more_states.iter() {
                states.insert(state.clone());
            }
            self.discover_states_and_rules(&mut states)
        }
    }
}

