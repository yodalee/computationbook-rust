use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use skicombinator::{SKICombinator};

#[derive(Clone)]
pub enum SKI {
    SKISymbol(String),
    SKICall(Box<SKI>, Box<SKI>),
    SKICombinator(SKICombinator),
}

impl SKI {
    pub fn s() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::s())) }
    pub fn k() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::k())) }
    pub fn i() -> Box<SKI> { Box::new(SKI::SKICombinator(SKICombinator::i())) }

    pub fn skisymbol(name: &str) -> Box<SKI> {
        if name == "S" || name == "K" || name == "I" {
            panic!("S, K, I are preserved for SKICombinator");
        }

        Box::new(SKI::SKISymbol(name.to_string()))
    }
    pub fn skicall(l: Box<SKI>, r: Box<SKI>) -> Box<SKI> {
        Box::new(SKI::SKICall(l, r))
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

    pub fn reducible(&self) -> bool {
        match *self {
            SKI::SKISymbol(_) => false,
            SKI::SKICall(ref l, ref r) => l.reducible() || r.reducible() || self.combinator().callable(self.arguments()),
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

    pub fn as_function_of(&self, name: &str) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(ref n) if n == "S" || n == "K" || n == "I" => {
                SKI::skicall(SKI::skisymbol("K"), Box::new(self.clone()))
            },
            SKI::SKISymbol(ref n) => {
                if n == name { SKI::skisymbol("I") }
                else { SKI::skicall(SKI::skisymbol("K"), Box::new(self.clone())) }
            },
            SKI::SKICall(ref l, ref r) => {
                SKI::skicall(
                    SKI::skicall(
                        SKI::skisymbol("S"),
                        l.as_function_of(name),
                    ),
                r.as_function_of(name),
                )
            },
            _ => panic!(),
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
