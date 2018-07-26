use std::collections::HashSet;
use std::hash::{Hash};
use std::fmt::Debug;

use super::farule::{FARule};
use super::nfadesign::{NFADesign};
use super::stateset::{StateSet};
use super::dfadesign::{DFADesign};
use super::dfarulebook::{DFARulebook};

pub struct NFASimulation<T> {
    nfa_design: NFADesign<T>
}

impl<T: Debug + Eq + Clone + Hash + Ord> NFASimulation<T> {
    pub fn new(nfa_design: &NFADesign<T>) -> NFASimulation<T> {
        NFASimulation {
            nfa_design: nfa_design.clone()
        }
    }

    pub fn next_state (&self, states: &HashSet<T>, c: char) -> StateSet<T> {
        let mut nfa = self.nfa_design.to_nfa_with_state(states);
        nfa.read_character(c);
        StateSet::new(&nfa.current_state())
    }

    pub fn rule_for(&self, states: &HashSet<T>) -> Vec<FARule<StateSet<T>>> {
        let start_set = StateSet::new(states);
        let alphabet = self.nfa_design.rulebook().alphabet();
        alphabet.iter()
                .map(|&c| FARule::new(&start_set, c, &self.next_state(states, c)))
                .collect()
    }

    pub fn discover_states_and_rules(&self, states: &HashSet<StateSet<T>>)
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
            let mut union = states.union(&more_states).cloned().collect();
            self.discover_states_and_rules(&mut union)
        }
    }

    pub fn to_dfa_design(&self) -> DFADesign<StateSet<T>> {
        let start_state = self.nfa_design.to_nfa().current_state();
        let mut start_set = HashSet::new();
        start_set.insert(StateSet::new(&start_state));
        let (states, rules) = self.discover_states_and_rules(&mut start_set);
        let accept_state = states.into_iter()
                                 .filter(|state| self.nfa_design.to_nfa_with_state(&state.0).accepting())
                                 .collect();
        DFADesign::new(StateSet::new(&start_state), &accept_state, &DFARulebook::new(rules))
    }
}

