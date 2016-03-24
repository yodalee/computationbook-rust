mod regex;
mod tonfa;
mod farule;
mod nfadesign;
mod nfarulebook;
mod nfa;
mod helper;

use regex::{Regex};
use tonfa::{ToNFA};

pub fn main() {
    let pattern = Regex::repeat(Regex::choose(Regex::concatenate(Regex::literal('a'), Regex::literal('b')), Regex::literal('a')));
    println!("{}", pattern);

    let mut nfadesign = Regex::empty().to_nfa_design();
    println!("{}", nfadesign.accept(""));
    println!("{}", nfadesign.accept("a"));
    nfadesign = Regex::literal('a').to_nfa_design();
    println!("{}", nfadesign.accept(""));
    println!("{}", nfadesign.accept("a"));
    println!("{}", nfadesign.accept("b"));
    println!("{}", Regex::empty().matches("a"));
    println!("{}", Regex::literal('a').matches("a"));
}
