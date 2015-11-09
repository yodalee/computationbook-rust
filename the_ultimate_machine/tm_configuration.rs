use std::fmt;
use tape::{Tape};

pub struct TMConfiguration {
    pub state: i32,
    pub tape: Tape,
}

impl fmt::Display for TMConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<TMConfiguration state={} tape={}>",
            self.state,
            self.tape,
        )
    }
}
impl TMConfiguration {
    pub fn new(state: i32, tape: Tape) -> TMConfiguration {
        TMConfiguration {state: state, tape: tape}
    }
}
