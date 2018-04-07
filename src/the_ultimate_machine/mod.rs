pub mod tape;
pub mod tm_configuration;
pub mod tm_rule;
pub mod dtm_rulebook;
pub mod dtm;

#[cfg(test)]
mod tests {
    use super::tape::{Tape};
    use super::tm_configuration::{TMConfiguration};
    use super::tm_rule::{TMRule, Direction};
    use super::dtm_rulebook::{DTMRulebook};
    use super::dtm::{DTM};
    use std::collections::HashSet;

    #[test]
    fn test_tm_tape() {
        let mut tape = Tape::new("101", '1', "", '_');

        assert_eq!("#<Tape 101(1)>", format!("{}", tape));
        assert_eq!('1', tape.middle);
        tape.move_head_left();
        tape.write('0');
        tape.move_head_right();
        tape.move_head_right();
        tape.write('0');
        assert_eq!("#<Tape 1001(0)>", format!("{}", tape));
        assert_eq!('0', tape.middle);
    }

    #[test]
    fn test_tm_rule() {
        let rule = TMRule::new(1, '0', 2, '1', Direction::Right);
        assert!(rule.applies_to(&TMConfiguration::new(1, Tape::new("", '0', "", '_'))));
        assert!(!rule.applies_to(&TMConfiguration::new(1, Tape::new("", '1', "", '_'))));
        assert!(!rule.applies_to(&TMConfiguration::new(2, Tape::new("", '0', "", '_'))));
    }

    #[test]
    fn test_tm_config() {
        let rule = TMRule::new(1, '0', 2, '1', Direction::Right);
        let mut config = TMConfiguration::new(1, Tape::new("", '0', "", '_'));
        rule.follow(&mut config);
        assert_eq!(config.state, 2);
        assert_eq!(format!("{}", config.tape), "#<Tape 1(_)>");
    }

    #[test]
    fn test_tm_rulebook() {
        let tape = Tape::new("101", '1', "", '_');
        let rulebook = DTMRulebook::new(vec!(
            TMRule::new(1, '0', 2, '1', Direction::Right),
            TMRule::new(1, '1', 1, '0', Direction::Left),
            TMRule::new(1, '_', 2, '1', Direction::Right),
            TMRule::new(2, '0', 2, '0', Direction::Right),
            TMRule::new(2, '1', 2, '1', Direction::Right),
            TMRule::new(2, '_', 3, '_', Direction::Left)
        ));
        let mut config = TMConfiguration::new(1, tape);
        assert_eq!(config.state, 1);
        assert_eq!(format!("{}", config.tape), "#<Tape 101(1)>");

        rulebook.next_configuration(&mut config);
        assert_eq!(config.state, 1);
        assert_eq!(format!("{}", config.tape), "#<Tape 10(1)0>");

        rulebook.next_configuration(&mut config);
        assert_eq!(config.state, 1);
        assert_eq!(format!("{}", config.tape), "#<Tape 1(0)00>");

        rulebook.next_configuration(&mut config);
        assert_eq!(config.state, 2);
        assert_eq!(format!("{}", config.tape), "#<Tape 11(0)0>");
    }

    #[test]
    fn test_dtm() {
        let tape = Tape::new("101", '1', "", '_');
        let rulebook = DTMRulebook::new(vec!(
            TMRule::new(1, '0', 2, '1', Direction::Right),
            TMRule::new(1, '1', 1, '0', Direction::Left),
            TMRule::new(1, '_', 2, '1', Direction::Right),
            TMRule::new(2, '0', 2, '0', Direction::Right),
            TMRule::new(2, '1', 2, '1', Direction::Right),
            TMRule::new(2, '_', 3, '_', Direction::Left)
        ));
        let mut accepting_state = HashSet::new();
        accepting_state.insert(3);
        let mut dtm = DTM::new(TMConfiguration::new(1, tape), accepting_state, rulebook);

        assert_eq!(dtm.current_configuration.state, 1);
        assert_eq!(format!("{}", dtm.current_configuration.tape), "#<Tape 101(1)>");
        assert!(!dtm.accepting());

        dtm.step();
        assert_eq!(dtm.current_configuration.state, 1);
        assert_eq!(format!("{}", dtm.current_configuration.tape), "#<Tape 10(1)0>");
        assert!(!dtm.accepting());

        dtm.run();
        assert_eq!(dtm.current_configuration.state, 3);
        assert_eq!(format!("{}", dtm.current_configuration.tape), "#<Tape 110(0)_>");
        assert!(dtm.accepting());
    }

    #[test]
    fn test_dtm_stuck() {
        let tape = Tape::new("121", '1', "", '_');
        let rulebook = DTMRulebook::new(vec!(
            TMRule::new(1, '0', 2, '1', Direction::Right),
            TMRule::new(1, '1', 1, '0', Direction::Left),
            TMRule::new(1, '_', 2, '1', Direction::Right),
            TMRule::new(2, '0', 2, '0', Direction::Right),
            TMRule::new(2, '1', 2, '1', Direction::Right),
            TMRule::new(2, '_', 3, '_', Direction::Left)
        ));
        let mut accepting_state = HashSet::new();
        accepting_state.insert(3);
        let mut dtm = DTM::new(TMConfiguration::new(1, tape), accepting_state, rulebook);
        dtm.run();

        assert_eq!(dtm.current_configuration.state, 1);
        assert_eq!(format!("{}", dtm.current_configuration.tape), "#<Tape 1(2)00>");
        assert!(!dtm.accepting());
        assert!(dtm.stuck());
    }
}
