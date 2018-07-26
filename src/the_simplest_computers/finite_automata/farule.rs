use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug,Clone)]
pub struct FARule<T> {
    pub state: T,
    pub character: char,
    pub next_state: T,
}

impl<T: Eq + PartialEq + Clone> FARule<T> {
    pub fn new(state: &T, character: char, next_state: &T) -> Self {
        FARule {
            state: state.clone(),
            character: character,
            next_state: next_state.clone()
        }
    }

    pub fn applies_to(&self, state: &T, character: char) -> bool {
        self.state == *state && self.character == character
    }

    pub fn follow(&self) -> T {
        self.next_state.clone()
    }
}

impl<T: Display> Display for FARule<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "FARule {} --{}--> {}", self.state, self.character, self.next_state)
    }
}
