pub mod pdarule;
pub mod pdaconfiguration;
pub mod dpdarulebook;
pub mod dpda;
pub mod dpdadesign;
pub mod npdarulebook;
pub mod npda;
pub mod npdadesign;

#[cfg(test)]
mod tests {
    use super::pdarule::{PDARule};
    use super::pdaconfiguration::{PDAConfiguration};
    use super::dpdarulebook::{DPDARulebook};
    use super::dpda::{DPDA};
    use super::dpdadesign::{DPDADesign};
    use super::npdarulebook::{NPDARulebook};
    use super::npda::{NPDA};
    use super::npdadesign::{NPDADesign};
    use helper::{to_hashset};

    #[test]
    fn test_dpa_config() {
        let rule = PDARule::new(1, '(', 2, '$', &['b', '$']);
        println!("{}", rule);
        let config = PDAConfiguration::new(1, &['$']);
        println!("{}", config);
        assert!(rule.applies_to(&config, '('));
        println!("{}", rule.follow(&config));
    }

    #[test]
    fn test_dpa_rulebook() {
        let mut config = PDAConfiguration::new(1, &['$']);
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        config = rulebook.next_config(&config, '(');
        println!("{}", config);
        assert_eq!(config.stack, &['$', 'b']);
        config = rulebook.next_config(&config, '(');
        println!("{}", config);
        assert_eq!(config.stack, &['$', 'b', 'b']);
        config = rulebook.next_config(&config, ')');
        println!("{}", config);
        assert_eq!(config.stack, &['$', 'b']);
    }

    #[test]
    fn test_dpda_string() {
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        let mut dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
        assert!(dpda.accept());
        dpda.read_string("(()");
        assert!(!dpda.accept());
        assert_eq!(dpda.config.stack, &['$', 'b']);
    }

    #[test]
    fn test_freemove() {
        let mut config = PDAConfiguration::new(2, &['$']);
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        assert_eq!(config.state, Some(2));
        config = rulebook.follow_free_moves(&config);
        assert_eq!(config.state, Some(1));

        /* blow up
        DPDARulebook::new(&[PDARule::new(1, '\0', 1, '$', &['$'])])
            .follow_free_moves(&PDAConfiguration::new(1, &['$']));
        */
    }

    #[test]
    fn test_dpda() {
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        let mut dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
        dpda.read_string("(()(");
        assert!(!dpda.accept());
        assert_eq!(dpda.current_config().state, Some(2));
        assert_eq!(dpda.current_config().stack, &['$', 'b', 'b']);
        dpda.read_string("))()");
        assert!(dpda.accept());
        assert_eq!(dpda.current_config().state, Some(1));
        assert_eq!(dpda.current_config().stack, &['$']);
    }

    #[test]
    fn test_dpda_design() {
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        let dpda_design = DPDADesign::new(1, '$', &[1], &rulebook);
        assert!(dpda_design.accept("(((((((((())))))))))"));
        assert!(dpda_design.accept("()(())((()))(()(()))"));
        assert!(!dpda_design.accept("(()(()(()()(()()))()"));
        assert!(!dpda_design.accept("())"));
    }

    #[test]
    fn test_dpda_stuck() {
        let rulebook = DPDARulebook::new(
            &[PDARule::new(1, '(', 2, '$', &['b', '$']),
              PDARule::new(2, '(', 2, 'b', &['b', 'b']),
              PDARule::new(2, ')', 2, 'b', &[]),
              PDARule::new(2, '\0', 1, '$', &['$'])]);
        let mut dpda = DPDA::new(&PDAConfiguration::new(1, &['$']), &[1], &rulebook);
        dpda.read_string("())");
        assert_eq!(dpda.config.state, None);
        assert!(!dpda.accept());
        assert!(dpda.is_stuck());

        let dpda_design = DPDADesign::new(1, '$', &[1], &rulebook);
        assert!(!dpda_design.accept("())"));
    }

    #[test]
    fn test_dpda_stringab() {
        let rulebook = DPDARulebook::new(&[
            PDARule::new(1, 'a', 2, '$', &['a', '$']),
            PDARule::new(1, 'b', 2, '$', &['b', '$']),
            PDARule::new(2, 'a', 2, 'a', &['a', 'a']),
            PDARule::new(2, 'b', 2, 'b', &['b', 'b']),
            PDARule::new(2, 'a', 2, 'b', &[]),
            PDARule::new(2, 'b', 2, 'a', &[]),
            PDARule::new(2, '\0', 1, '$', &['$'])
        ]);
        let dpda_design = DPDADesign::new(1, '$', &[1], &rulebook);
        assert!(dpda_design.accept("ababab"));
        assert!(dpda_design.accept("bbbaaaab"));
        assert!(!dpda_design.accept("baa"));
    }

    #[test]
    fn test_dpda_palindrome_stringab() {
        let rulebook = DPDARulebook::new(&[
            PDARule::new(1, 'a', 1, '$', &['a', '$']),
            PDARule::new(1, 'a', 1, 'a', &['a', 'a']),
            PDARule::new(1, 'a', 1, 'b', &['a', 'b']),
            PDARule::new(1, 'b', 1, '$', &['b', '$']),
            PDARule::new(1, 'b', 1, 'a', &['b', 'a']),
            PDARule::new(1, 'b', 1, 'b', &['b', 'b']),
            PDARule::new(1, 'm', 2, '$', &['$']),
            PDARule::new(1, 'm', 2, 'a', &['a']),
            PDARule::new(1, 'm', 2, 'b', &['b']),
            PDARule::new(2, 'a', 2, 'a', &[]),
            PDARule::new(2, 'b', 2, 'b', &[]),
            PDARule::new(2, '\0', 3, '$', &['$'])
        ]);
        let dpda_design = DPDADesign::new(1, '$', &[3], &rulebook);
        println!("demo: palindrome with m in middle");
        assert!(dpda_design.accept("abmba"));
        assert!(dpda_design.accept("babbamabbab"));
        assert!(!dpda_design.accept("abmb"));
        assert!(!dpda_design.accept("baambaa"));
    }

    #[test]
    fn test_npda() {
        let rulebook = NPDARulebook::new(&[
            PDARule::new(1, 'a', 1, '$', &['a', '$']),
            PDARule::new(1, 'a', 1, 'a', &['a', 'a']),
            PDARule::new(1, 'a', 1, 'b', &['a', 'b']),
            PDARule::new(1, 'b', 1, '$', &['b', '$']),
            PDARule::new(1, 'b', 1, 'a', &['b', 'a']),
            PDARule::new(1, 'b', 1, 'b', &['b', 'b']),
            PDARule::new(1, '\0', 2, '$', &['$']),
            PDARule::new(1, '\0', 2, 'a', &['a']),
            PDARule::new(1, '\0', 2, 'b', &['b']),
            PDARule::new(2, 'a', 2, 'a', &[]),
            PDARule::new(2, 'b', 2, 'b', &[]),
            PDARule::new(2, '\0', 3, '$', &['$'])
        ]);
        let config = PDAConfiguration::new(1, &['$']);
        let mut npda = NPDA::new(&to_hashset(&[config]), &to_hashset(&[3]), &rulebook);

        assert!(npda.accept());
        for config in npda.current_config().iter() {
            println!("{}", config);
        }

        npda.read_string("abb");
        assert!(!npda.accept());
        for config in npda.current_config().iter() {
            println!("{}", config);
        }

        npda.read_character('a');
        assert!(npda.accept());
        for config in npda.current_config().iter() {
            println!("{}", config);
        }
    }

    #[test]
    fn test_npda_palindrome() {
        let rulebook = NPDARulebook::new(&[
            PDARule::new(1, 'a', 1, '$', &['a', '$']),
            PDARule::new(1, 'a', 1, 'a', &['a', 'a']),
            PDARule::new(1, 'a', 1, 'b', &['a', 'b']),
            PDARule::new(1, 'b', 1, '$', &['b', '$']),
            PDARule::new(1, 'b', 1, 'a', &['b', 'a']),
            PDARule::new(1, 'b', 1, 'b', &['b', 'b']),
            PDARule::new(1, '\0', 2, '$', &['$']),
            PDARule::new(1, '\0', 2, 'a', &['a']),
            PDARule::new(1, '\0', 2, 'b', &['b']),
            PDARule::new(2, 'a', 2, 'a', &[]),
            PDARule::new(2, 'b', 2, 'b', &[]),
            PDARule::new(2, '\0', 3, '$', &['$'])
        ]);
        let npda_design = NPDADesign::new(1, '$', &[3], &rulebook);
        assert!(npda_design.accept("abba"));
        assert!(npda_design.accept("babbaabbab"));
        assert!(!npda_design.accept("abb"));
        assert!(!npda_design.accept("baabaa"));
    }
}

pub fn main() {
}
