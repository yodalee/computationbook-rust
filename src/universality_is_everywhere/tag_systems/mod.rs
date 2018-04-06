pub mod tag_rulebook;
pub mod tag_system;
pub mod tag_rule;

#[cfg(test)]
mod tests {
    use super::tag_rule::*;
    use super::tag_rulebook::*;
    use super::tag_system::*;

    fn print_assert(expr: &str, name: &str, ans: &str) {
        let fmtstr = format!("{}", expr);
        println!("{}: {}", name, fmtstr);
        assert_eq!(ans, fmtstr);
    }

    #[test]
    fn test_tagsystem_double_string() {
        let rulebook = TagRulebook::new(2, vec!(
                TagRule::new('a', "aa"),
                TagRule::new('b', "bbbb")));
        let mut system = TagSystem::new("aabbbbbb", rulebook);
        for _ in 0..4 {
            println!("{}", system.current_string);
            system.step()
        }
        print_assert(&system.current_string, "double string", "aabbbbbbbbbbbb");
    }

    #[test]
    fn test_tagsystem_double_and_stoppable() {
        let rulebook = TagRulebook::new(2, vec!(
                TagRule::new('a', "cc"),
                TagRule::new('b', "dddd")));
        let mut system = TagSystem::new("aabbbbbb", rulebook);
        system.run();
        print_assert(&system.current_string, "double and stop", "ccdddddddddddd");
    }

    #[test]
    fn test_tagsystem_half_number() {
        let rulebook = TagRulebook::new(2, vec!(
                TagRule::new('a', "cc"),
                TagRule::new('b', "d")));
        let mut system = TagSystem::new("aabbbbbbbbbbbb", rulebook);
        system.run();
        print_assert(&system.current_string, "half and stop", "ccdddddd");
    }

    #[test]
    fn test_tagsystem_increment_input() {
        let rulebook = TagRulebook::new(2, vec!(
                TagRule::new('a', "ccdd"),
                TagRule::new('b', "dd")));
        let mut system = TagSystem::new("aabbbb", rulebook);
        system.run();
        print_assert(&system.current_string, "increment", "ccdddddd");
    }

    #[test]
    fn test_tagsystem_double_and_increment() {
        let rulebook = TagRulebook::new(2, vec!(
            // double
            TagRule::new('a', "cc"),
            TagRule::new('b', "dddd"),
            // increment
            TagRule::new('c', "eeff"),
            TagRule::new('d', "ff")));
        let mut system = TagSystem::new("aabbbb", rulebook);
        system.run();
        print_assert(&system.current_string, "double and increment", "eeffffffffff");
    }

    #[test]
    fn test_tagsystem_even_or_odd() {
        let rulebook = TagRulebook::new(2, vec!(
            TagRule::new('a', "cc"),
            TagRule::new('b', "d"),
            TagRule::new('c', "eo"),
            TagRule::new('d', ""),
            TagRule::new('e', "e")));
        let mut system = TagSystem::new("aabbbbbbbb", rulebook.clone());
        system.run();
        print_assert(&system.current_string, "input 4 (even)", "e");
        system = TagSystem::new("aabbbbbbbbbb", rulebook);
        system.run();
        print_assert(&system.current_string, "input 5 (odd)", "o");
    }
}
