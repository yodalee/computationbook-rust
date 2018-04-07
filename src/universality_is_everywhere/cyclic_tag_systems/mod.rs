mod cyclic_tag_rule;
mod cyclic_tag_rulebook;
mod cyclic_tag_system;
mod tag_to_cyclic;

#[cfg(test)]
mod tests {
    use universality_is_everywhere::tag_systems::tag_rule::{TagRule};
    use universality_is_everywhere::tag_systems::tag_rulebook::{TagRulebook};
    use universality_is_everywhere::tag_systems::tag_system::{TagSystem};

    use super::cyclic_tag_rule::{CyclicTagRule};
    use super::cyclic_tag_rulebook::{CyclicTagRulebook};
    use super::cyclic_tag_system::{CyclicTagSystem};
    use super::tag_to_cyclic::{TagToCyclic};

    fn print_assert(expr: &str, name: &str, ans: &str) {
        let fmtstr = format!("{}", expr);
        println!("{}: {}", name, fmtstr);
        assert_eq!(ans, fmtstr);
    }

    #[test]
    fn test_cyclic_demo() {
        let rulebook = CyclicTagRulebook::new(&[CyclicTagRule::new("1"), CyclicTagRule::new("0010"), CyclicTagRule::new("10")].to_vec());
        let mut system = CyclicTagSystem::new("11", rulebook);
        for _ in 0..16 {
            println!("{}", system.current_string);
            system.step();
        }
        print_assert(&system.current_string, "period 16 times", "00101");
        for _ in 0..20 {
            println!("{}", system.current_string);
            system.step();
        }
        print_assert(&system.current_string, "period +20 times", "101");
    }

    #[test]
    fn test_cyclic_alphabet() {
        let tagrulebook = TagRulebook::new(2, [TagRule::new('a', "ccdd"), TagRule::new('b', "dd")].to_vec());
        let tagsystem = TagSystem::new("aabbbb", tagrulebook);
        assert_eq!(tagsystem.alphabet(), &['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_cyclic_encoder() {
        let tagrulebook = TagRulebook::new(2, [TagRule::new('a', "ccdd"), TagRule::new('b', "dd")].to_vec());
        let tagsystem = TagSystem::new("aabbbb", tagrulebook);
        let encoder = tagsystem.encoder();
        assert_eq!("0010", encoder.encode_character('c'));
        assert_eq!("001010000100", encoder.encode_string("cab"));
    }

    #[test]
    fn test_cyclic_tocyclic() {
        let tagrulebook = TagRulebook::new(2, [TagRule::new('a', "ccdd"), TagRule::new('b', "dd")].to_vec());
        let tagsystem = TagSystem::new("aabbbb", tagrulebook);
        let encoder = tagsystem.encoder();
        let rule = tagsystem.rulebook().rules()[0].clone();
        let cycrule = rule.to_cyclic(&encoder);
        println!("{}", cycrule); //rule.to_cyclic(&encoder));
        assert_eq!(format!("{}", cycrule), "<CyclicTagRule 0010001000010001>");

        println!("{:?}", tagsystem.rulebook().cyclic_rules(&encoder));
        println!("{:?}", tagsystem.rulebook().cyclic_padding_rules(&encoder));

        let mut cyclic_system = tagsystem.to_cyclic(&encoder);

        cyclic_system.run();
    }
}
