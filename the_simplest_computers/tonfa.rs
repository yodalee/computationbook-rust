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
                let accept_state = second.accept_state().iter().map(|x| x+first.size).collect::<Vec<u32>>();

                let mut rule1 = first.rules();
                let mut rule2 = second.rules();
                for r in rule2.iter_mut() { r.shift(first.size) }

                let mut extrarule = first.accept_state()
                    .iter().map(|state|
                        FARule::new(*state, '\0', second.start_state()))
                    .collect::<Vec<FARule>>();

                rule1.extend_from_slice(&rule2);
                rule1.extend_from_slice(&extrarule);

                NFADesign::new(
                    start_state,
                    &toHashSet(&accept_state),
                    &NFARulebook::new(rule1)
                )
            },
            _ => panic!("XD"),
            //Regex::Concatenate(ref l, ref r) => {},
            //Regex::Choose(ref l, ref r) => {},
            //Regex::Repeat(ref p) => {},
        }
    }

    fn matches(&self, s: &str) -> bool {
        match *self {
            _ => self.to_nfa_design().accept(s)
        }
    }
}
