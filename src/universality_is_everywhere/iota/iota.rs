use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use super::ski::{SKI};

pub trait SKICombinator: Display {
    fn box_clone(&self) -> Box<SKICombinator>;
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI>;
    fn callable(&self, args: Vec<Box<SKI>>) -> bool;
    fn to_iota(&self) -> Box<SKI>;
}

impl Clone for Box<SKICombinator> {
    fn clone(&self) -> Box<SKICombinator> {
        self.box_clone()
    }
}

pub fn s() -> Box<SKICombinator> { Box::new(S{}) }
pub fn k() -> Box<SKICombinator> { Box::new(K{}) }
pub fn i() -> Box<SKICombinator> { Box::new(I{}) }
pub fn iota() -> Box<SKICombinator> { Box::new(IOTA{}) }

// S combinator
#[derive(Clone)]
pub struct S {}


impl SKICombinator for S {
    fn box_clone(&self) -> Box<SKICombinator> {
        Box::new((*self).clone())
    }
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI> {
        SKI::skicall(
            SKI::skicall(args[0].clone(), args[2].clone()),
            SKI::skicall(args[1].clone(), args[2].clone())
        )
    }
    fn callable(&self, args: Vec<Box<SKI>>) -> bool {
        args.len() == 3
    }
    fn to_iota(&self) -> Box<SKI> {
        SKI::skicall(SKI::iota(),
        SKI::skicall(SKI::iota(),
        SKI::skicall(SKI::iota(),
        SKI::skicall(SKI::iota(), SKI::iota()))))
    }
}

impl Display for S {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "S")
    }
}

// K combinator
#[derive(Clone)]
pub struct K {}

impl SKICombinator for K {
    fn box_clone(&self) -> Box<SKICombinator> {
        Box::new((*self).clone())
    }
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI> {
        args[0].clone()
    }
    fn callable(&self, args: Vec<Box<SKI>>) -> bool {
        args.len() == 2
    }
    fn to_iota(&self) -> Box<SKI> {
        SKI::skicall(SKI::iota(),
        SKI::skicall(SKI::iota(),
        SKI::skicall(SKI::iota(), SKI::iota())))
    }
}

impl Display for K {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "K")
    }
}

// I combinator
#[derive(Clone)]
pub struct I {}

impl SKICombinator for I {
    fn box_clone(&self) -> Box<SKICombinator> {
        Box::new((*self).clone())
    }
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI> {
        args[0].clone()
    }
    fn callable(&self, args: Vec<Box<SKI>>) -> bool {
        args.len() == 1
    }
    fn to_iota(&self) -> Box<SKI> {
        SKI::skicall(SKI::iota(), SKI::iota())
    }
}

impl Display for I {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "I")
    }
}

// IOTA combinator
#[derive(Clone)]
pub struct IOTA {}

impl SKICombinator for IOTA {
    fn box_clone(&self) -> Box<SKICombinator> {
        Box::new((*self).clone())
    }
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI> {
        SKI::skicall(SKI::skicall(args[0].clone(), SKI::s()), SKI::k())
    }
    fn callable(&self, args: Vec<Box<SKI>>) -> bool {
        args.len() == 1
    }
    fn to_iota(&self) -> Box<SKI> {
        SKI::iota()
    }
}

impl Display for IOTA {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ɩ")
    }
}

