use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use syntax::{Node};
use context::{Context};

#[derive(Clone, PartialEq)]
pub enum Type {
    Number,
    Boolean,
    Void,
}

pub trait SyntaxType {
    fn get_type(&self, context: &mut Context) -> Option<Type>;
}

impl SyntaxType for Node {
    fn get_type(&self, context: &mut Context) -> Option<Type> {
        match *self {
            Node::Number(_) => Some(Type::Number),
            Node::Boolean(_) => Some(Type::Boolean),
            Node::Add(ref l, ref r) | Node::Multiply(ref l, ref r) => {
                match (l.get_type(context), r.get_type(context)) {
                    (Some(Type::Number), Some(Type::Number)) => Some(Type::Number),
                    _ => None,
                }
            }
            Node::LessThan(ref l, ref r) => {
                match (l.get_type(context), r.get_type(context)) {
                    (Some(Type::Number), Some(Type::Number)) => Some(Type::Boolean),
                    _ => None,
                }
            }
            Node::Variable(ref name) => {
                context.get(name)
            }
            Node::DoNothing => {
                Some(Type::Void)
            }
            Node::Sequence(ref first, ref second) => {
                match (first.get_type(context), second.get_type(context)) {
                    (Some(Type::Void), Some(Type::Void)) => Some(Type::Void),
                    _ => None,
                }
            }
            Node::If(ref cond, ref conse, ref alter) => {
                match (cond.get_type(context), conse.get_type(context), alter.get_type(context)) {
                    (Some(Type::Boolean), Some(Type::Void), Some(Type::Void)) => Some(Type::Void),
                    _ => None,
                }
            }
            Node::While(ref cond, ref body) => {
                match (cond.get_type(context), body.get_type(context)) {
                    (Some(Type::Boolean), Some(Type::Void)) => Some(Type::Void),
                    _ => None,
                }
            }
            Node::Assign(ref name, ref expr) => {
                let rtype = expr.get_type(context);
                let ltype = context.get(name);
                match (ltype, rtype) {
                    (Some(ref lt), Some(ref rt)) if lt == rt => Some(Type::Void),
                    _ => None,
                }
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Type::Number => write!(f, "<Type Number>"),
            Type::Boolean => write!(f, "<Type Boolean>"),
            Type::Void => write!(f, "<Type Void>"),
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Type::Number => write!(f, "<Type Number>"),
            Type::Boolean => write!(f, "<Type Boolean>"),
            Type::Void => write!(f, "<Type Void>"),
        }
    }
}
