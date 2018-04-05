use super::sign::{Sign};

pub trait HasSign {
    fn sign(self) -> Sign;
}

impl HasSign for i32 {
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
