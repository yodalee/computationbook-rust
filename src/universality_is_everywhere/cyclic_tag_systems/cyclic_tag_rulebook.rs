use super::cyclic_tag_rule::{CyclicTagRule};

const DELETION_NUMBER: usize = 1;

pub struct CyclicTagRulebook {
    rules: Vec<CyclicTagRule>,
    it: usize,
}

impl CyclicTagRulebook {
    pub fn new(rules: &Vec<CyclicTagRule>) -> Self {
        CyclicTagRulebook {
            rules: rules.clone(),
            it: 0,
        }
    }

    pub fn applies_to(&self, s: &str) -> bool {
        s.len() >= DELETION_NUMBER
    }

    pub fn next_string(&mut self, s: &str) -> String {
        let newstr = self.follow_next_rule(s);
        unsafe {
            newstr.get_unchecked(DELETION_NUMBER..newstr.len())
                     .to_string()
        }
    }

    pub fn follow_next_rule(&mut self, s: &str) -> String {
        let ref rule = self.rules[self.it];
        self.it = self.it + 1;
        if self.it == self.rules.len() {
            self.it = 0;
        }
        if rule.applies_to(s) {
            rule.follow(s)
        } else {
            s.to_string()
        }
    }
}
