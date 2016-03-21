use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

#[derive(Clone)]
pub enum SKI {
    SKISymbol(String),
    SKICall(Box<SKI>, Box<SKI>),
}

impl SKI {
    pub fn skisymbol(name: &str) -> Box<SKI> {
        Box::new(SKI::SKISymbol(name.to_string()))
    }
    pub fn skicall(l: Box<SKI>, r: Box<SKI>) -> Box<SKI> {
        Box::new(SKI::SKICall(l, r))
    }
}

impl Display for SKI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKI::SKICall(ref l, ref r) => write!(f, "{0}[{1}]", l, r),
            SKI::SKISymbol(ref name) => write!(f, "{}", *name),
        }
    }
}
