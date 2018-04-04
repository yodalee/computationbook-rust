use std::rc::Rc;

use the_simplest_computers::helper::{to_hashset};

use the_simplest_computers::finite_automata::farule::{FARule};
use the_simplest_computers::finite_automata::nfadesign::{NFADesign};
use the_simplest_computers::finite_automata::nfarulebook::{NFARulebook};
use super::regex::{Regex};
use super::state::{State, RCState};

pub trait ToNFA {
    fn to_nfa_design(&self) -> NFADesign<RCState>;
    fn matches(&self, &str) -> bool;
}

impl ToNFA for Regex {
    fn to_nfa_design(&self) -> NFADesign<RCState> {
        match *self {
            Regex::Empty => {
                let start_state = Rc::new(State{});
                NFADesign::new(
                    &start_state,
                    &to_hashset(&[start_state.clone()]),
                    &NFARulebook::new(vec![])
                )
            },
            Regex::Literal(c) => {
                let start_state = Rc::new(State{});
                let accept_state = Rc::new(State{});
                let rule = FARule::new(&start_state, c, &accept_state);
                NFADesign::new(
                    &start_state,
                    &to_hashset(&[accept_state]),
                    &NFARulebook::new(
                        vec![rule]),
                )
            },
            Regex::Concatenate(ref l, ref r) => {
                let first = l.to_nfa_design();
                let second = r.to_nfa_design();
                let start_state = first.start_state();
                let accept_state = second.accept_state();
                let mut rule1 = first.rules();
                let rule2 = second.rules();
                let extrarules = first.accept_state().iter()
                    .map(|state| FARule::new(state, '\0', &second.start_state()))
                    .collect::<Vec<FARule<RCState>>>();
                rule1.extend_from_slice(&rule2);
                rule1.extend_from_slice(&extrarules);
                NFADesign::new(
                    &start_state,
                    &accept_state,
                    &NFARulebook::new(rule1))
            },
            Regex::Choose(ref l, ref r) => {
                let first = l.to_nfa_design();
                let second = r.to_nfa_design();
                let start_state = Rc::new(State{});
                let accept_state = first.accept_state().union(&second.accept_state()).cloned().collect();
                let mut rules = first.rules();
                rules.extend_from_slice(&second.rules());
                rules.extend_from_slice(&[
                    FARule::new(&start_state, '\0', &first.start_state()),
                    FARule::new(&start_state, '\0', &second.start_state())]);
                NFADesign::new(
                    &start_state,
                    &accept_state,
                    &NFARulebook::new(rules))
            },
            Regex::Repeat(ref p) => {
                let pattern_nfa = p.to_nfa_design();
                let start_state = Rc::new(State{});
                let mut accept_state = pattern_nfa.accept_state();
                accept_state.insert(start_state.clone());

                let mut rules = pattern_nfa.rules();
                rules.extend(accept_state.iter().map(|state| FARule::new(state, '\0', &pattern_nfa.start_state())));

                NFADesign::new(
                    &start_state,
                    &accept_state,
                    &NFARulebook::new(rules))
            },
        }
    }

    fn matches(&self, s: &str) -> bool {
        match *self {
            _ => self.to_nfa_design().accept(s)
        }
    }
}
