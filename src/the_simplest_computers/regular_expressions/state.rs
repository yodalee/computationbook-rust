use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

#[derive(Eq, Hash)]
pub struct State;
pub type RCState = Rc<State>;

impl PartialEq for State {
    fn eq(&self, rhs: &Self) -> bool {
        self as *const _ == rhs as *const _
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self as *const _ as usize)
    }
}

