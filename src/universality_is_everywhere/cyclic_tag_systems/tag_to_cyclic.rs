use universality_is_everywhere::tag_systems::tag_rule::{TagRule};
use universality_is_everywhere::tag_systems::tag_rulebook::{TagRulebook};
use universality_is_everywhere::tag_systems::tag_system::{TagSystem};

use super::cyclic_tag_rule::{CyclicTagRule};
use super::cyclic_tag_rulebook::{CyclicTagRulebook};
use super::cyclic_tag_system::{CyclicTagSystem};

pub struct CyclicTagEncoder {
    pub v: Vec<char>,
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
    fn alphabet(&self) -> Vec<char>;
}

impl TagToCyclic for TagRule {
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.append_characters().chars().collect();
        v.push(self.first_char());
        v.sort();
        v.dedup();
        v
    }
}

impl TagRule {
    pub fn to_cyclic(&self, encoder: &CyclicTagEncoder) -> CyclicTagRule {
        CyclicTagRule::new(&encoder.encode_string(&self.append_characters()))
    }
}

impl TagToCyclic for TagRulebook {
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.rules()
            .iter()
            .flat_map(|r| r.alphabet().into_iter())
            .collect();
        v.sort();
        v.dedup();
        v
    }
}

impl TagRulebook {
    pub fn to_cyclic(&self, encoder: &CyclicTagEncoder) -> CyclicTagRulebook {
        let mut v:Vec<CyclicTagRule> = self.cyclic_rules(encoder);
        v.extend(self.cyclic_padding_rules(encoder));
        CyclicTagRulebook::new(&v)
    }
    pub fn cyclic_rules(&self, encoder: &CyclicTagEncoder) -> Vec<CyclicTagRule> {
        encoder.v.iter().map(|&c| self.cyclic_rule_for(c, encoder)).collect()
    }

    pub fn cyclic_rule_for(&self, c: char, encoder: &CyclicTagEncoder) -> CyclicTagRule {
        match self.rule_for(&c.to_string()) {
            Some(r) => r.to_cyclic(encoder),
            None => CyclicTagRule::new(""),
        }
    }

    pub fn cyclic_padding_rules(&self, encoder: &CyclicTagEncoder) -> Vec<CyclicTagRule> {
        vec![CyclicTagRule::new(""); encoder.v.len() * (self.deletion_number()-1) as usize]

    }
}

impl TagToCyclic for TagSystem {
    fn alphabet(&self) -> Vec<char> {
        let mut v = self.rulebook().alphabet();
        v.append(&mut self.current_string.chars().collect());
        v.sort();
        v.dedup();
        v
    }
}

impl TagSystem {
    pub fn to_cyclic(&self, encoder: &CyclicTagEncoder) -> CyclicTagSystem {
        CyclicTagSystem::new(
            &encoder.encode_string(&self.current_string),
            self.rulebook().to_cyclic(encoder)
            )
    }
    pub fn encoder(&self) -> CyclicTagEncoder {
        CyclicTagEncoder::new(&self.alphabet())
    }
}
