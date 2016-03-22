mod regex;

use regex::{Regex};

pub fn main() {
    let pattern = Regex::repeat(Regex::choose(Regex::concatenate(Regex::literal('a'), Regex::literal('b')), Regex::literal('a')));
    println!("{}", pattern);
}


