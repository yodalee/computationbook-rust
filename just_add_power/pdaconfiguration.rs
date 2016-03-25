use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

pub struct PDAConfiguration {
    pub state: u32,
    pub stack: Vec<char>,
}

impl PDAConfiguration {
    pub fn new(state: u32, stack: &[char]) -> Self {
        PDAConfiguration{state: state, stack: stack.to_vec()}
    }
}

impl Display for PDAConfiguration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "PDAConfiguration state={}, stack={}",
               self.state, self.stack.iter().rev().cloned().collect::<String>())
    }
}
