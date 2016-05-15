use tag_rule::{TagRule};

pub struct TagRulebook {
    pub deletion_number: i32,
    pub rules: Vec<TagRule>,
}

impl TagRulebook {
    pub fn new(deletion_number: i32, rules: Vec<TagRule>) -> TagRulebook {
        TagRulebook{
            deletion_number: deletion_number,
            rules: rules,
        }
    }

    pub fn next_string(&self, input: &str) -> String {
        match self.rule_for(input) {
            Some(rule) => unsafe {
                let newString = rule.follow(input);
                newString.slice_unchecked(self.deletion_number as usize, newString.len())
                    .to_string()
            },
            None => "".to_string(),
        }
    }

    pub fn rule_for(&self, input: &str) -> Option<&TagRule> {
        self.rules.iter().find(|rule| rule.applies_to(input))
    }

    pub fn applies_to(&self, input: &str) -> bool {
        input.len() >= self.deletion_number as usize &&
            self.rule_for(input).is_some()
    }
}
