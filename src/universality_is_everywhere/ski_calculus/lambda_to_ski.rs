use programming_with_nothing::lambda::lambda::{Lambda};

use super::ski::{SKI};

pub trait toSKI {
    fn to_ski(&self) -> Box<SKI>;
}

impl toSKI for Lambda {
    fn to_ski(&self) -> Box<SKI> {
        match *self {
            Lambda::LCVariable(ref name) => SKI::skisymbol(&name),
            Lambda::LCCall(ref l, ref r) => SKI::skicall(l.to_ski(), r.to_ski()),
            Lambda::LCFunction(ref param, ref body) => body.to_ski().as_function_of(&param),
        }
    }
}

