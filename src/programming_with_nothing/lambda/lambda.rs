use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone)]
pub enum Lambda {
    LCVariable(String),
    LCFunction(String, Box<Lambda>),
    LCCall(Box<Lambda>, Box<Lambda>),
}

impl Lambda {
    pub fn lcvar(s: &str) -> Box<Lambda> {
        Box::new(Lambda::LCVariable(s.to_string()))
    }
    pub fn lcfun(s: &str, l: Box<Lambda>) -> Box<Lambda> {
        Box::new(Lambda::LCFunction(s.to_string(), l.clone()))
    }
    pub fn lccall(l: Box<Lambda>, r: Box<Lambda>) -> Box<Lambda> {
        Box::new(Lambda::LCCall(l.clone(), r.clone()))
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Lambda::LCVariable(ref name) => write!(f, "{}", name),
            Lambda::LCFunction(ref param, ref body) => write!(f, "-> {} {{ {} }}", param, body),
            Lambda::LCCall(ref l, ref r) => write!(f, "{}[{}]", l, r),
        }
    }
}
