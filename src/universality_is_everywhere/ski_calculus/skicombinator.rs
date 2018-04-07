use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use super::ski::{SKI};

pub trait SKICombinator: Display {
    fn box_clone(&self) -> Box<SKICombinator>;
    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI>;
    fn callable(&self, args: Vec<Box<SKI>>) -> bool;
}

impl Clone for Box<SKICombinator> {
    fn clone(&self) -> Box<SKICombinator> {
        self.box_clone()
    }
}

pub fn s() -> Box<SKICombinator> { Box::new(S{}) }
pub fn k() -> Box<SKICombinator> { Box::new(K{}) }
pub fn i() -> Box<SKICombinator> { Box::new(I{}) }

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
}

impl Display for I {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "I")
    }
}
