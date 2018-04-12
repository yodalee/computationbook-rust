use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

use universality_is_everywhere::ski_calculus::ski::{SKI};
use universality_is_everywhere::ski_calculus::skicombinator::{SKICombinator};

pub fn iota() -> Box<SKI> { Box::new(SKI::SKICombinator(Box::new(IOTA{}))) }

// iota combinator
#[derive(Clone)]
pub struct IOTA {}

impl SKICombinator for IOTA {
    fn name(&self) -> char { 'ɩ' }

    fn box_clone(&self) -> Box<SKICombinator> {
        Box::new((*self).clone())
    }

    fn call(&self, args: Vec<Box<SKI>>) -> Box<SKI> {
        SKI::skicall(SKI::skicall(args[0].clone(), SKI::s()), SKI::k())
    }

    fn callable(&self, args: Vec<Box<SKI>>) -> bool {
        args.len() == 1
    }
}

impl Display for IOTA {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ɩ")
    }
}

