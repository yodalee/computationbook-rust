use super::tag_rulebook::{TagRulebook};

pub struct TagSystem {
    pub current_string: String,
    rulebook: TagRulebook,
}

impl TagSystem {
    pub fn new<S: Into<String>>(current_string: S, rulebook: TagRulebook) -> TagSystem {
        TagSystem {
            current_string: current_string.into(),
            rulebook: rulebook,
        }
    }

    pub fn step(&mut self) {
        self.current_string = self.rulebook.next_string(&self.current_string).to_string();
    }

    pub fn run(&mut self) {
        while self.rulebook.applies_to(&self.current_string) {
            println!("{}", self.current_string);
            self.step();
        }

        println!("{}", self.current_string);
    }
}
