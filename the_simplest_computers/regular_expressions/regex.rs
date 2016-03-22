use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

pub enum Regex {
    Empty,
    Literal(char),
    Concatenate(Box<Regex>, Box<Regex>),
    Choose(Box<Regex>, Box<Regex>),
    Repeat(Box<Regex>),
}

impl Regex {
    pub fn empty()-> Box<Regex> { Box::new(Regex::Empty) }
    pub fn literal(c: char)-> Box<Regex> { Box::new(Regex::Literal(c)) }
    pub fn concatenate(l: Box<Regex>, r: Box<Regex>)-> Box<Regex> { Box::new(Regex::Concatenate(l, r)) }
    pub fn choose(l: Box<Regex>, r: Box<Regex>)-> Box<Regex> { Box::new(Regex::Choose(l, r)) }
    pub fn repeat(p: Box<Regex>)-> Box<Regex> { Box::new(Regex::Repeat(p)) }

    fn bracket(&self, outer_precedence: u32) -> String {
        if self.precedence() < outer_precedence {
            format!("({})", self)
        } else {
            format!("{}", self)
        }
    }

    fn precedence(&self) -> u32 {
        match *self {
            Regex::Empty | Regex::Literal(_) => 3,
            Regex::Concatenate(_,_) => 1,
            Regex::Choose(_,_) => 0,
            Regex::Repeat(_) => 2,
        }
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Regex::Empty => write!(f, ""),
            Regex::Literal(s) => write!(f, "{}", s),
            Regex::Concatenate(ref l, ref r) => write!(f, "{}", [l, r].iter().map(|pat| pat.bracket(self.precedence())).collect::<Vec<String>>().join("")),
            Regex::Choose(ref l, ref r) => write!(f, "{}", [l, r].iter().map(|pat| pat.bracket(self.precedence())).collect::<Vec<String>>().join("|")),
            Regex::Repeat(ref p) => write!(f, "{}*", p.bracket(self.precedence())),
        }
    }
}
