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
        panic!("Disable Now");
    }

    fn matches(&self, s: &str) -> bool {
        match *self {
            _ => self.to_nfa_design().accept(s)
        }
    }
}
