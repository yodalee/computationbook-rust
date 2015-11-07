use tape::{Tape};

pub struct TMConfiguration {
    pub State: i32,
    pub Tape: Tape,
}

impl TMConfiguration {
    pub fn new(state: i32, tape: Tape) -> TMConfiguration {
        TMConfiguration {State: state, Tape: tape}
    }
}
