use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

use pdaconfiguration::{PDAConfiguration};

#[derive(Clone)]
pub struct PDARule {
    state: u32,
    character: char,
    next_state: u32,
    pop_char: char,
    push_char: Vec<char>,
}

impl PDARule {
    pub fn new(state: u32, character: char, next_state: u32, pop_char: char, push_char: &[char]) -> Self {
        PDARule{
            state: state,
            character: character,
            next_state: next_state,
            pop_char: pop_char,
            push_char: push_char.to_vec()}
    }
    pub fn applies_to(&self, config: &PDAConfiguration, c: char) -> bool {
        match config.stack.last() {
            Some(&top) => { self.state == config.state && self.pop_char == top && self.character == c },
            None => false,
        }
    }

    pub fn follow(&self, config: &PDAConfiguration) -> PDAConfiguration {
        PDAConfiguration::new(self.next_state, &self.next_stack(config))
    }

    pub fn next_stack(&self, config: &PDAConfiguration) -> Vec<char> {
        let mut stack = config.stack.clone();
        stack.pop();
        for c in self.push_char.iter().rev() {
            stack.push(*c)
        }
        stack
    }
}

impl Display for PDARule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PDARule state={}, character={}, next_state={}, pop_char={}, push_char={}",
               self.state, self.character, self.next_state, self.pop_char, self.push_char.iter().cloned().collect::<String>())
    }
}
