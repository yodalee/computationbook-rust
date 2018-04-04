use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

use the_simplest_computers::regular_expressions::state::{State};

#[derive(Clone)]
pub struct FARule {
    pub state: Rc<State>,
    character: char,
    pub next_state: Rc<State>,
}

impl FARule {
    pub fn new(state: &Rc<State>, character: char, next_state: &Rc<State>) -> Self {
        FARule{state: state.clone(), character: character, next_state: next_state.clone()}
    }

    pub fn applies_to(&self, state: &Rc<State>, character: char) -> bool {
        self.state == *state && self.character == character
    }

    pub fn follow(&self) -> Rc<State> {
        self.next_state.clone()
    }
}

impl Display for FARule {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "FARule {} --{}--> {}", self.state, self.character, self.next_state)
    }
}
