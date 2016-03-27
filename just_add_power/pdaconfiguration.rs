use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PDAConfiguration {
    pub state: Option<u32>,
    pub stack: Vec<char>,
}

impl PDAConfiguration {
    pub fn new(state: u32, stack: &[char]) -> Self {
        PDAConfiguration{state: Some(state), stack: stack.to_vec()}
    }

    pub fn stuck(&self) -> Self {
        PDAConfiguration{state: None, stack: self.stack.clone()}
    }

    pub fn is_stuck(&self) -> bool {
        self.state.is_none()
    }
}

impl Display for PDAConfiguration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.state {
            Some(u) => write!(f, "PDAConfiguration state={}, stack={}",
                u, self.stack.iter().rev().cloned().collect::<String>()),
            None => write!(f, "PDAConfiguration state=stuck, stack={}",
                self.stack.iter().rev().cloned().collect::<String>()),
        }
    }
}
