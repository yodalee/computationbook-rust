use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use super::iota::{SKICombinator, s, k, i, iota};

#[derive(Clone)]
pub enum SKI {
    SKISymbol(String),
    SKICall(Box<SKI>, Box<SKI>),
    SKICombinator(Box<SKICombinator>),
}

impl SKI {
    pub fn skisymbol(name: &str) -> Box<SKI> {
        // we don't check name == "S", "K", "I" "other" here
        // It's user's responsibility not to use confusing name
        // The print will give something like S[S] while the second
        // S is a symbol instead of combinator
        Box::new(SKI::SKISymbol(name.to_string()))
    }

    pub fn skicall(l: Box<SKI>, r: Box<SKI>) -> Box<SKI> {
        Box::new(SKI::SKICall(l, r))
    }

    pub fn s() -> Box<SKI> { Box::new(SKI::SKICombinator(s())) }
    pub fn k() -> Box<SKI> { Box::new(SKI::SKICombinator(k())) }
    pub fn i() -> Box<SKI> { Box::new(SKI::SKICombinator(i())) }
    pub fn iota() -> Box<SKI> { Box::new(SKI::SKICombinator(iota())) }

    pub fn left(&self) -> Box<SKI> {
        match *self {
            SKI::SKICall(ref l, _) => l.clone(),
            _ => panic!("Type has no left: {}", *self)
        }
    }
    pub fn right(&self) -> Box<SKI> {
        match *self {
            SKI::SKICall(_, ref r) => r.clone(),
            _ => panic!("Type has no left: {}", *self)
        }
    }

    pub fn call(&self, arg: Vec<Box<SKI>>) -> Box<SKI> {
        match *self {
            SKI::SKICombinator(ref c) => c.call(arg),
            _ => panic!("Only SKICombinator is callable")
        }
    }

    pub fn callable(&self, arg: Vec<Box<SKI>>) -> bool {
        match *self {
            SKI::SKICombinator(ref c) => c.callable(arg),
            _ => false,
        }
    }

    pub fn combinator(&self) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(_) | SKI::SKICombinator(_) => Box::new(self.clone()),
            SKI::SKICall(ref l, _) => l.combinator(),
        }
    }

    pub fn arguments(&self) -> Vec<Box<SKI>> {
        match *self {
            SKI::SKISymbol(_) | SKI::SKICombinator(_) => {
                Vec::new()
            },
            SKI::SKICall(ref l, ref r) => {
                let mut arg = l.arguments();
                arg.push(r.clone());
                arg
            },
        }
    }

    pub fn reducible(&self) -> bool {
        match *self {
            SKI::SKICall(ref l, ref r) =>
                l.reducible() || r.reducible() ||
                self.combinator().callable(self.arguments()),
            _ => false,
        }
    }

    pub fn reduce(&self) -> Box<SKI> {
        match *self {
            SKI::SKICall(ref l, ref r) => {
                if l.reducible() { SKI::skicall(l.reduce(), r.clone()) }
                else if r.reducible() { SKI::skicall(l.clone(), r.reduce()) }
                else { self.combinator().call(self.arguments()) }
            },
            _ => panic!("Cannot reduce on type: {}", *self)
        }
    }

    pub fn to_iota(&self) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(_) => Box::new(self.clone()),
            SKI::SKICall(ref l, ref r) => SKI::skicall(l.to_iota(), r.to_iota()),
            SKI::SKICombinator(ref c) => c.to_iota(),
        }
    }
}

impl Display for SKI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKI::SKICall(ref l, ref r) => write!(f, "{0}[{1}]", l, r),
            SKI::SKISymbol(ref name) => write!(f, "{}", *name),
            SKI::SKICombinator(ref c) => write!(f, "{}", c),
        }
    }
}
