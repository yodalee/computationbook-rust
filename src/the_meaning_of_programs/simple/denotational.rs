use super::syntax::{Node};

pub trait Denotational {
    fn to_ruby(&self) -> String;
}

impl Denotational for Node {
    fn to_ruby(&self) -> String {
        match *self {
            Node::Number(v) => format!("-> e {{ {} }}", v),
            Node::Boolean(v) => format!("-> e {{ {} }}", v),
            Node::Variable(ref name) => format!("-> e {{ e[:{}] }}", name),
            Node::Add(ref l, ref r) => format!("-> e {{ ({}).call(e) + ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::Multiply(ref l, ref r) => format!("-> e {{ ({}).call(e) * ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::LessThan(ref l, ref r) => format!("-> e {{ ({}).call(e) < ({}).call(e) }}", l.to_ruby(), r.to_ruby()),
            Node::Assign(ref name, ref expr) => format!("-> e {{ e.merge({{ :{} => ({}).call(e) }}) }}", name, expr.to_ruby()),
            Node::DoNothing => "-> e { e }".to_string(),
            Node::If(ref condition, ref consequence, ref alternative) => format!(
                "-> e {{if ({}).call(e) then ({}).call(e) else ({}).call(e) end }}",
                condition.to_ruby(), consequence.to_ruby(), alternative.to_ruby()
                ),
            Node::Sequence(ref first, ref second) => format!("-> e {{ ({}).call(({}).call(e)) }}", second.to_ruby(), first.to_ruby()),
            Node::While(ref condition, ref body) => format!("-> e {{ while ({}).call(e); e = ({}).call(e); end; e }}", condition.to_ruby(), body.to_ruby()),
        }
    }
}
