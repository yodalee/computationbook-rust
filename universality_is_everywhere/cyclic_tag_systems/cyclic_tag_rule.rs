use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use tag_rule::{TagRule};

const FIRST_CHARACTER: char = '1';

#[derive(Debug, Clone)]
pub struct CyclicTagRule {
    rule: TagRule,
}

impl CyclicTagRule {
    pub fn new(append_characters: &str) -> Self {
        CyclicTagRule {
            rule: TagRule::new(FIRST_CHARACTER, append_characters)
        }
    }

    pub fn applies_to(&self, s: &str) -> bool {
        self.rule.applies_to(s)
    }

    pub fn follow(&self, s: &str) -> String {
        self.rule.follow(s)
    }
}

impl Display for CyclicTagRule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<CyclicTagRule {}>", self.rule.append_characters)
    }
}
