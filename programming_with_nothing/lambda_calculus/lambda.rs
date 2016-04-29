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
    pub fn lcvar(s: &str) -> Box<Lambda> { Box::new(Lambda::LCVariable(s.to_string())) }
    pub fn lcfun(s: &str, l: Box<Lambda>) -> Box<Lambda> { Box::new(Lambda::LCFunction(s.to_string(), l)) }
    pub fn lccall(l: Box<Lambda>, r: Box<Lambda>) -> Box<Lambda> { Box::new(Lambda::LCCall(l, r)) }
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

pub trait Reduce {
    fn replace(&self, &str, &Box<Lambda>) -> Box<Lambda>;
}

impl Reduce for Lambda {
    fn replace(&self, name: &str, replacement: &Box<Lambda>) -> Box<Lambda> {
        match *self {
            Lambda::LCVariable(ref varname) => {
                if varname == name {
                    replacement.clone()
                } else {
                    Box::new(self.clone())
                }
            },
            Lambda::LCFunction(ref param, ref body) => {
                if param == name {
                    Box::new(self.clone())
                } else {
                    Lambda::lcfun(param, body.replace(name, replacement))
                }
            },
            Lambda::LCCall(ref l, ref r) => Lambda::lccall(l.replace(name, replacement), r.replace(name, replacement))
        }
    }
}
