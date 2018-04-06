use programming_with_nothing::lambda::lambda::{Lambda};

use super::ski::{SKI};

pub trait AsFunction {
    fn as_function_of(&self, name: &str) -> Box<SKI>;
}

impl AsFunction for SKI {
    fn as_function_of(&self, name: &str) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(ref n) => {
                if n == name { SKI::i() }
                else { SKI::skicall(SKI::k(), Box::new(self.clone())) }
            },
            SKI::SKICall(ref l, ref r) => {
                let lfun = l.as_function_of(name);
                let rfun = r.as_function_of(name);
                SKI::skicall(SKI::skicall(SKI::s(), lfun), rfun)
            },
            SKI::SKICombinator(ref c) => {
                SKI::skicall(SKI::k(), Box::new(SKI::SKICombinator(c.clone())))
            },
        }
    }
}

pub trait ToSKI {
    fn to_ski(&self) -> Box<SKI>;
}

impl ToSKI for Lambda {
    fn to_ski(&self) -> Box<SKI> {
        match *self {
            Lambda::LCVariable(ref name) => SKI::skisymbol(&name),
            Lambda::LCCall(ref l, ref r) => SKI::skicall(l.to_ski(), r.to_ski()),
            Lambda::LCFunction(ref param, ref body) => body.to_ski().as_function_of(&param),
        }
    }
}

