use std::fmt;
use tape::{Tape};

pub struct TMConfiguration {
    pub State: i32,
    pub Tape: Tape,
}

impl fmt::Display for TMConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#<TMConfiguration state={} tape={}>",
            self.State,
            self.Tape,
        )
    }
}
impl TMConfiguration {
    pub fn new(state: i32, tape: Tape) -> TMConfiguration {
        TMConfiguration {State: state, Tape: tape}
    }
}
