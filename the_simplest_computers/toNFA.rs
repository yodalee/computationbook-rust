use std::collections::HashSet;

use nfadesign::{NFADesign};
use regex::{Regex};

trait toNFA {
    fn to_nfa_design(&self) -> NFADesign;
}

fn toHashSet(arr: &[u32]) -> HashSet<u32> {
    arr.iter().cloned().collect::<HashSet<u32>>()
}

impl toNFA for Regex {
    fn to_nfa_design(&self) {
        match *self {
            Regex::Empty => write!(f, ""),
            Regex::Literal(s) => write!(f, "{}", s),
            Regex::Concatenate(ref l, ref r) => write!(f, "{}", [l, r].iter().map(|pat| pat.bracket(self.precedence())).collect::<Vec<String>>().join("")),
            Regex::Choose(ref l, ref r) => write!(f, "{}", [l, r].iter().map(|pat| pat.bracket(self.precedence())).collect::<Vec<String>>().join("|")),
            Regex::Repeat(ref p) => write!(f, "{}*", p.bracket(self.precedence())),
        }
        NFADesign::new(start_state, &toHashSet(accept_state), rulebook)
    }
}
