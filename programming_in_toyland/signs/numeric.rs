use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use sign::{Sign};

pub trait hasSign {
    fn sign(self) -> Sign;
}

impl hasSign for i32 {
    fn sign(self) -> Sign {
        if self < 0 {
            Sign::NEGATIVE
        } else if self == 0 {
            Sign::ZERO
        } else {
            Sign::POSITIVE
        }
    }
}
