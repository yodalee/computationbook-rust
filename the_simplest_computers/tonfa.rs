use std::collections::HashSet;

use nfadesign::{NFADesign};
use nfarulebook::{NFARulebook};
use regex::{Regex};
use helper::{toHashSet};
use farule::{FARule};

pub trait ToNFA {
    fn to_nfa_design(&self) -> NFADesign;
    fn matches(&self, &str) -> bool;
}

impl ToNFA for Regex {
    fn to_nfa_design(&self) -> NFADesign {
        match *self {
            Regex::Empty => {
                NFADesign::new(
                    0,
                    &toHashSet(&[0]),
                    &NFARulebook::new(vec![])
                )
            },
            Regex::Literal(c) => {
                NFADesign::new(
                    0,
                    &toHashSet(&[1]),
                    &NFARulebook::new(vec![FARule::new(0, c, 1)])
                )
            },
            Regex::Concatenate(ref l, ref r) => {
                let first = l.to_nfa_design();
                let second = r.to_nfa_design();
                let start_state = first.start_state();
                let accept_state = second.accept_state().iter().map(|x| x+first.size).collect::<HashSet<u32>>();

                let mut rule1 = first.rules();
                let mut rule2 = second.rules();
                for r in rule2.iter_mut() { r.shift(first.size) }

                let extrarule = first.accept_state()
                    .iter().map(|state|
                        FARule::new(*state, '\0', second.start_state()))
                    .collect::<Vec<FARule>>();

                rule1.extend_from_slice(&rule2);
                rule1.extend_from_slice(&extrarule);

                NFADesign::new(
                    start_state,
                    &accept_state,
                    &NFARulebook::new(rule1)
                )
            },
            Regex::Choose(ref l, ref r) => {
                let first = l.to_nfa_design();
                let second = r.to_nfa_design();
                let start_state = 0;
                let accept_state1 = first.accept_state().iter().map(|x| x+1).collect::<HashSet<u32>>();
                let accept_state2 = second.accept_state().iter().map(|x| x+first.size+1).collect::<HashSet<u32>>();
                let accept_state = accept_state1.union(&accept_state2).cloned().collect::<HashSet<u32>>();

                let mut rule1 = first.rules();
                for r in rule1.iter_mut() { r.shift(1) }
                let mut rule2 = second.rules();
                for r in rule2.iter_mut() { r.shift(1+first.size) }
                let extrarule = vec![FARule::new(0, '\0', 1), FARule::new(0, '\0', first.size+1)];
                rule1.extend_from_slice(&rule2);
                rule1.extend_from_slice(&extrarule);

                NFADesign::new(
                    start_state,
                    &accept_state,
                    &NFARulebook::new(rule1)
                )
            },
            Regex::Repeat(ref p) => {
                let pattern_nfa = p.to_nfa_design();

                let start_state = 0;
                let mut accept_state = pattern_nfa.accept_state()
                    .iter().map(|x| x+1).collect::<HashSet<u32>>();
                let mut rules = pattern_nfa.rules();
                for r in rules.iter_mut() { r.shift(1) }

                rules.extend(accept_state.iter().map(|x| FARule::new(*x, '\0', start_state)));
                rules.push(FARule::new(start_state, '\0', 1));

                accept_state.insert(start_state);

                NFADesign::new(
                    start_state,
                    &accept_state,
                    &NFARulebook::new(rules)
                )
            },
        }
    }

    fn matches(&self, s: &str) -> bool {
        match *self {
            _ => self.to_nfa_design().accept(s)
        }
    }
}
