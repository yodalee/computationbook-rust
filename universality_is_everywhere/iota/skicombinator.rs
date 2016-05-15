use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use ski::{SKI};

#[derive(Clone)]
pub enum SKICombinator {
    S,
    K,
    I,
    Iota,
}

impl SKICombinator {
    pub fn s() -> SKICombinator { SKICombinator::S }
    pub fn k() -> SKICombinator { SKICombinator::K }
    pub fn i() -> SKICombinator { SKICombinator::I }
    pub fn iota() -> SKICombinator { SKICombinator::Iota }

    pub fn to_iota(&self) -> Box<SKI> {
        match *self {
            SKICombinator::S => {
                SKI::skicall(SKI::iota(),
                SKI::skicall(SKI::iota(),
                SKI::skicall(SKI::iota(),
                SKI::skicall(SKI::iota(), SKI::iota()))))
            }
            SKICombinator::K => {
                SKI::skicall(SKI::iota(),
                SKI::skicall(SKI::iota(),
                SKI::skicall(SKI::iota(), SKI::iota())))
            }
            SKICombinator::I => {
                SKI::skicall(SKI::iota(), SKI::iota())
            }
            SKICombinator::Iota => { SKI::iota() }
        }
    }

    pub fn as_function_of(&self, name: &str) -> Box<SKI> {
        SKI::skicall(SKI::k(),
            Box::new(SKI::SKICombinator(self.clone())))
    }

    pub fn call(&self, arg: Vec<Box<SKI>>) -> Box<SKI> {
        match *self {
            SKICombinator::S => {
                SKI::skicall(
                    SKI::skicall(arg[0].clone(), arg[2].clone()),
                    SKI::skicall(arg[1].clone(), arg[2].clone())
                )
            },
            SKICombinator::K | SKICombinator::I => {
                arg[0].clone()
            },
            SKICombinator::Iota => {
                SKI::skicall(SKI::skicall(arg[0].clone(), SKI::s()), SKI::k())
            }
        }
    }

    pub fn callable(&self, arg: Vec<Box<SKI>>) -> bool {
        match *self {
            SKICombinator::S => { arg.len() == 3 },
            SKICombinator::K => { arg.len() == 2 },
            SKICombinator::I => { arg.len() == 1 },
            SKICombinator::Iota => { arg.len() == 1 },
        }
    }

}

impl Display for SKICombinator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKICombinator::S => write!(f, "S"),
            SKICombinator::K => write!(f, "K"),
            SKICombinator::I => write!(f, "I"),
            SKICombinator::Iota => write!(f, "ɩ"),
        }
    }
}
