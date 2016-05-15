use cyclic_tag_rulebook::{CyclicTagRulebook};

pub struct CyclicTagSystem {
    pub current_string: String,
    rulebook: CyclicTagRulebook,
}

impl CyclicTagSystem {
    pub fn new(current_string: &str, rulebook: CyclicTagRulebook) -> Self {
        CyclicTagSystem {
            current_string: current_string.to_string(),
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
