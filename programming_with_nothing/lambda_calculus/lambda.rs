use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum Lambda {
    LCVariable(String),
    LCFunction(String, Box<Lambda>),
    LCCall(Box<Lambda>, Box<Lambda>),
}

impl Lambda {
    pub fn lcvar(s: &str) -> Box<Lambda> { Box::new(Lambda::LCVariable(s.to_string())) }
    pub fn lcfun(s: &str, l: Box<Lambda>) -> Box<Lambda> { Box::new(Lambda::LCFunction(s.to_string(), l)) }
    pub fn lccall(l: Box<Lambda>, r: Box<Lambda>) -> Box<Lambda> { Box::new(Lambda::LCCall(l, r)) }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Lambda::LCVariable(ref name) => write!(f, "{}", name),
            Lambda::LCFunction(ref name, ref l) => write!(f, "-> {} {{ {} }}", name, l),
            Lambda::LCCall(ref l, ref r) => write!(f, "{}[{}]", l, r),
        }
    }
}
