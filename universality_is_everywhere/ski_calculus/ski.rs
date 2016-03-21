use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

pub enum SKICombinator {
    S,
    K,
    I,
}

impl SKICombinator {
    fn s() -> Box<SKICombinator> { Box::new(SKICombinator::S) }
    fn k() -> Box<SKICombinator> { Box::new(SKICombinator::K) }
    fn i() -> Box<SKICombinator> { Box::new(SKICombinator::I) }
}

pub enum SKI {
    SKICall(Box<SKI>, Box<SKI>),
    SKISymbol(String),
    SKICombinator(Box<SKICombinator>),
}

impl SKI {
    pub fn skicall(l: Box<SKI>, r: Box<SKI>) -> Box<SKI> {
        Box::new(SKI::SKICall(l, r))
    }
    pub fn skisymbol(name: &str) -> Box<SKI> { Box::new(SKI::SKISymbol(name.to_string())) }
    pub fn s() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::s())) }
    pub fn k() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::k())) }
    pub fn i() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::i())) }
}

impl Display for SKI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKI::SKICall(ref l, ref r) => write!(f, "{0}[{1}]", l, r),
            SKI::SKISymbol(ref name) => write!(f, "{}", *name),
            SKI::SKICombinator(ref i) => write!(f, "{}", i),
        }
    }
}

impl Display for SKICombinator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKICombinator::S => write!(f, "S"),
            SKICombinator::K => write!(f, "K"),
            SKICombinator::I => write!(f, "I"),
        }
    }
}
