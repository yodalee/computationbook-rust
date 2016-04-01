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
    let mut pattern = Regex::repeat(Regex::choose(Regex::concatenate(Regex::literal('a'), Regex::literal('b')), Regex::literal('a')));
    println!("{}", pattern);
}
