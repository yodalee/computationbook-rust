use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;

#[derive(Clone)]
pub enum SKI {
    SKISymbol(String),
    SKICall(Box<SKI>, Box<SKI>),
}

impl SKI {
    pub fn skisymbol(name: &str) -> Box<SKI> {
        Box::new(SKI::SKISymbol(name.to_string()))
    }
    pub fn skicall(l: Box<SKI>, r: Box<SKI>) -> Box<SKI> {
        Box::new(SKI::SKICall(l, r))
    }

    pub fn call(&self, arg: Vec<Box<SKI>>) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(ref name) if name == "S" => {
                SKI::skicall(
                    SKI::skicall(arg[0].clone(), arg[2].clone()),
                    SKI::skicall(arg[1].clone(), arg[2].clone())
                )
            },
            SKI::SKISymbol(ref name) if name == "K" || name == "I" => {
                arg[0].clone()
            },
            _ => panic!("Only Symbol S, K, I is callable")
        }
    }

    pub fn callable(&self, arg: Vec<Box<SKI>>) -> bool {
        match *self {
            SKI::SKISymbol(ref name) if name == "S" => { arg.len() == 3 },
            SKI::SKISymbol(ref name) if name == "K" => { arg.len() == 2 },
            SKI::SKISymbol(ref name) if name == "I" => { arg.len() == 1 },
            _ => false,
        }
    }

    pub fn combinator(&self) -> Box<SKI> {
        match *self {
            SKI::SKISymbol(_) => Box::new(self.clone()),
            SKI::SKICall(ref l, ref r) => l.combinator()
        }
    }

    pub fn arguments(&self) -> Vec<Box<SKI>> {
        match *self {
            SKI::SKISymbol(_) => {
                Vec::new()
            },
            SKI::SKICall(ref l, ref r) => {
                let mut arg = l.arguments();
                arg.push(r.clone());
                arg
            }
        }
    }

    pub fn left(&self) -> Box<SKI> {
        match *self {
            SKI::SKICall(ref l, ref r) => l.clone(),
            _ => panic!("Type has no left: {}", *self)
        }
    }
    pub fn right(&self) -> Box<SKI> {
        match *self {
            SKI::SKICall(ref l, ref r) => r.clone(),
            _ => panic!("Type has no left: {}", *self)
        }
    }
}

impl Display for SKI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            SKI::SKICall(ref l, ref r) => write!(f, "{0}[{1}]", l, r),
            SKI::SKISymbol(ref name) => write!(f, "{}", *name),
        }
    }
}
