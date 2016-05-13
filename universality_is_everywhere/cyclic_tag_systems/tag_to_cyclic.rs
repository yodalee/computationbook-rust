use tag_rule::{TagRule};
use tag_rulebook::{TagRulebook};
use tag_system::{TagSystem};

use cyclic_tag_rule::{CyclicTagRule};

pub struct CyclicTagEncoder {
    v: Vec<char>,
}

impl CyclicTagEncoder {
    pub fn new(v: &Vec<char>) -> Self {
        CyclicTagEncoder { v: v.clone() }
    }
    pub fn encode_string(&self, s: &str) -> String {
        s.chars()
         .map(|c| self.encode_character(c) )
         .fold(String::new(), |s, sn| s+&sn)
    }

    pub fn encode_character(&self, c: char) -> String {
        let mut s:String = String::new();
        if let Some(i) = self.v.iter().position(|&x| x==c) {
            s = (0..self.v.len()).fold(String::new(), |s, n| s + if n == i {"1"} else {"0"});
        }
        return s
    }
}

pub trait TagToCyclic {
    fn to_cyclic(&self, encoder: CyclicTagEncoder) -> CyclicTagRule;
    fn alphabet(&self) -> Vec<char>;
    fn encoder(&self) -> CyclicTagEncoder;
}

impl TagToCyclic for TagRule {
    fn to_cyclic(&self, encoder: CyclicTagEncoder) -> CyclicTagRule {
        CyclicTagRule::new(&encoder.encode_string(&self.append_characters))
    }
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.append_characters.chars().collect();
        v.push(self.first_char);
        v.sort();
        v.dedup();
        v
    }
    fn encoder(&self) -> CyclicTagEncoder {
        panic!("cannot get encoder from type TagRule");
    }
}

impl TagToCyclic for TagRulebook {
    fn to_cyclic(&self, encoder: CyclicTagEncoder) -> CyclicTagRule {
        panic!("cannot generate cyclic tag rule from type TagRulebook");
    }
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.rules
            .iter()
            .flat_map(|r| r.alphabet().into_iter())
            .collect();
        v.sort();
        v.dedup();
        v
    }
    fn encoder(&self) -> CyclicTagEncoder {
        panic!("cannot get encoder from type TagRulebook");
    }
}

impl TagToCyclic for TagSystem {
    fn to_cyclic(&self, encoder: CyclicTagEncoder) -> CyclicTagRule {
        panic!("cannot generate cyclic tag rule from type TagSystem");
    }
    fn alphabet(&self) -> Vec<char> {
        let mut v = self.rulebook.alphabet();
        v.append(&mut self.current_string.chars().collect());
        v.sort();
        v.dedup();
        v
    }
    fn encoder(&self) -> CyclicTagEncoder {
        CyclicTagEncoder::new(&self.alphabet())
    }
}
