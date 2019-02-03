use super::tag_rule::{TagRule};

#[derive(Clone)]
pub struct TagRulebook {
    deletion_number: i32,
    rules: Vec<TagRule>,
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
                let newstr = rule.follow(input);
                newstr.get_unchecked(self.deletion_number as usize..newstr.len())
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

    pub fn deletion_number(&self) -> i32 { self.deletion_number }
    pub fn rules(&self) -> Vec<TagRule> { self.rules.clone() }
}
