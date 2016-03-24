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
