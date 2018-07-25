use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

use helper::{to_hashset, hashset_eq};

use super::farule::{FARule};
use super::nfadesign::{NFADesign};

#[derive(Clone)]
pub struct StateSet<T>(HashSet<T>);

impl<T: Clone> StateSet<T> {
    pub fn new(t: &HashSet<T>) -> Self{
        StateSet {
            0: t.clone()
        }
    }
}

impl<T: Eq + Clone + Hash> PartialEq for StateSet<T> {
    fn eq(&self, other: &StateSet<T>) -> bool {
        hashset_eq(&self.0, &other.0)
    }
}

impl<T: Eq + Clone + Hash> Eq for StateSet<T> {}

impl<T: Eq + Clone + Hash + Ord> Hash for StateSet<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&T> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

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

    pub fn rule_for(&self, states: &HashSet<T>) -> Vec<FARule<HashSet<T>>> {
        let alphabet = self.nfa_design.rulebook().alphabet();
        alphabet.iter().map(|&c| FARule::new(states, c, &self.next_state(states, c))).collect()
    }

    pub fn discover_states_and_rules(&self, mut states: &mut HashSet<StateSet<T>>) {
        let rules: Vec<_>  = states.iter()
                             .flat_map(|state| self.rule_for(&state.0))
                             .collect::<Vec<_>>();
        println!("{:?}", rules);
        let more_states: Vec<_> = rules.iter()
                                  .map(|rule| rule.follow())
                                  .collect();
        let mut more_states_set = HashSet::new();
        for stateset in more_states.iter() {
            more_states_set.insert(StateSet::new(&stateset));
        }
        if more_states_set.is_subset(states) {
            println!("XD");
        } else {
            for state in more_states_set.iter() {
                states.insert(state.clone());
            }
            self.discover_states_and_rules(&mut states);
        }
    }
}

