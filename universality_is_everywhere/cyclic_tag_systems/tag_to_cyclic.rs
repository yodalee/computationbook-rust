use tag_rule::{TagRule};
use tag_rulebook::{TagRulebook};
use tag_system::{TagSystem};

pub trait TagToCyclic {
    fn alphabet(&self) -> Vec<char>;
}

impl TagToCyclic for TagRule {
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.append_characters.chars().collect();
        v.push(self.first_char);
        v.sort();
        v.dedup();
        v
    }
}

impl TagToCyclic for TagRulebook {
    fn alphabet(&self) -> Vec<char> {
        let mut v: Vec<char> = self.rules
            .iter()
            .flat_map(|r| r.alphabet().into_iter())
            .collect();
        v.sort();
        v.dedup();
        v
    }
}

impl TagToCyclic for TagSystem {
    fn alphabet(&self) -> Vec<char> {
        let mut v = self.rulebook.alphabet();
        v.append(&mut self.current_string.chars().collect());
        v.sort();
        v.dedup();
        v
    }
}
