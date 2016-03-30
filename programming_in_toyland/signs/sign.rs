use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Neg;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

#[derive(PartialEq,Clone,Copy)]
pub enum Sign {
    POSITIVE,
    NEGATIVE,
    ZERO,
    UNKNOWN,
}

impl Neg for Sign {
    type Output = Sign;

    fn neg(self) -> Self {
        match self {
            Sign::POSITIVE => Sign::NEGATIVE,
            Sign::NEGATIVE => Sign::POSITIVE,
            _ => self,
        }
    }
}

impl Add for Sign {
    type Output = Sign;

    fn add(self, rhs: Self) -> Self {
        if self == rhs || rhs == Sign::ZERO {
            self
        } else if self == Sign::ZERO {
            rhs
        } else {
            Sign::UNKNOWN
        }
    }
}

impl Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self {
        if self == Sign::ZERO || rhs == Sign::ZERO {
            Sign::ZERO
        } else if self == Sign::UNKNOWN || rhs == Sign::UNKNOWN {
            Sign::UNKNOWN
        } else if self == rhs {
            Sign::POSITIVE
        } else {
            Sign::NEGATIVE
        }
    }
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Sign::UNKNOWN || other == &Sign::UNKNOWN {
            None
        } else if self == other {
            Some(Ordering::Equal)
        } else if self == &Sign::POSITIVE || (self == &Sign::ZERO && other == &Sign::NEGATIVE) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Sign::ZERO => write!(f, "<Sign zero>"),
            Sign::POSITIVE => write!(f, "<Sign positive>"),
            Sign::NEGATIVE => write!(f, "<Sign negative>"),
            Sign::UNKNOWN => write!(f, "<Sign unknown>"),
        }
    }
}
