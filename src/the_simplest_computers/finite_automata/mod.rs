pub mod farule;
pub mod dfarulebook;
pub mod dfa;
pub mod dfadesign;
pub mod nfarulebook;
pub mod nfa;
pub mod nfadesign;

#[cfg(test)]
mod tests {
    use super::farule::*;
    use super::dfarulebook::*;
    use super::dfa::*;
    use super::dfadesign::*;
    use super::nfarulebook::*;
    use super::nfa::*;
    use super::nfadesign::*;
    use helper::*;

    #[test]
    fn test_dfa_rulebook() {
        let rulebook = DFARulebook::new(
            vec![
                FARule::new(&1, 'a', &2), FARule::new(&1, 'b', &1),
                FARule::new(&2, 'a', &2), FARule::new(&2, 'b', &3),
                FARule::new(&3, 'a', &3), FARule::new(&3, 'b', &3)
            ]);
        assert_eq!(2, rulebook.next_state(&1, 'a'));
        assert_eq!(1, rulebook.next_state(&1, 'b'));
        assert_eq!(3, rulebook.next_state(&2, 'b'));
    }

    #[test]
    fn test_dfa() {
        let rulebook = DFARulebook::new(
            vec![
                FARule::new(&1, 'a', &2), FARule::new(&1, 'b', &1),
                FARule::new(&2, 'a', &2), FARule::new(&2, 'b', &3),
                FARule::new(&3, 'a', &3), FARule::new(&3, 'b', &3)
            ]);
        assert!(DFA::new(1, vec![1, 3], &rulebook).accepting());
        assert!(!DFA::new(1, vec![3], &rulebook).accepting());

        let mut dfa = DFA::new(1, vec![3], &rulebook);
        assert!(!dfa.accepting());
        dfa.read_character('b');
        assert!(!dfa.accepting());
        dfa.read_character('b');

        for _ in 0..3 {
            dfa.read_character('a')
        }

        assert!(!dfa.accepting());
        dfa.read_character('b');
        assert!(dfa.accepting());

        dfa = DFA::new(1, vec![3], &rulebook);
        assert!(!dfa.accepting());
        dfa.read_string("baaab");
        assert!(dfa.accepting());
    }

    #[test]
    fn test_dfa_design() {
        let rulebook = DFARulebook::new(
            vec![
                FARule::new(&1, 'a', &2), FARule::new(&1, 'b', &1),
                FARule::new(&2, 'a', &2), FARule::new(&2, 'b', &3),
                FARule::new(&3, 'a', &3), FARule::new(&3, 'b', &3)
            ]);
        let dfa_design = DFADesign::new(1, vec![3], &rulebook);
        assert!(!dfa_design.accept("a"));
        assert!(!dfa_design.accept("baa"));
        assert!(dfa_design.accept("baba"));
    }

    #[test]
    fn test_nfa_rulebook() {
        let rulebook = NFARulebook::new(
            vec![FARule::new(&1, 'a', &1), FARule::new(&1, 'b', &1),
                 FARule::new(&1, 'b', &2), FARule::new(&2, 'a', &3),
                 FARule::new(&2, 'b', &3), FARule::new(&3, 'a', &4),
                 FARule::new(&3, 'b', &4)
            ]);
        let result1 = rulebook.next_states(&to_hashset(&[1]), 'b');
        let ans1 = to_hashset(&[1,2]);
        let result2 = rulebook.next_states(&to_hashset(&[1,2]), 'a');
        let ans2 = to_hashset(&[1,3]);
        let result3 = rulebook.next_states(&to_hashset(&[1,3]), 'b');
        let ans3 = to_hashset(&[1,2,4]);
        assert!(result1.is_subset(&ans1) && result1.is_superset(&ans1));
        assert!(result2.is_subset(&ans2) && result2.is_superset(&ans2));
        assert!(result3.is_subset(&ans3) && result3.is_superset(&ans3));
    }

    #[test]
    fn test_nfa() {
        let rulebook = NFARulebook::new(
            vec![FARule::new(&1, 'a', &1), FARule::new(&1, 'b', &1),
                 FARule::new(&1, 'b', &2), FARule::new(&2, 'a', &3),
                 FARule::new(&2, 'b', &3), FARule::new(&3, 'a', &4),
                 FARule::new(&3, 'b', &4)
            ]);

        assert!(!NFA::new(&to_hashset(&[1]), &to_hashset(&[4]), &rulebook).accepting());
        assert!(NFA::new(&to_hashset(&[1,2,4]), &to_hashset(&[4]), &rulebook).accepting());

        let mut nfa = NFA::new(&to_hashset(&[1]), &to_hashset(&[4]), &rulebook);
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(!nfa.accepting());
        nfa.read_character('a');
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(nfa.accepting());

        nfa = NFA::new(&to_hashset(&[1]), &to_hashset(&[4]), &rulebook);
        assert!(!nfa.accepting());
        nfa.read_string("bbbbb");
        assert!(nfa.accepting());
    }

    #[test]
    fn test_nfadesign() {
        let rulebook = NFARulebook::new(
            vec![FARule::new(&1, 'a', &1), FARule::new(&1, 'b', &1),
                 FARule::new(&1, 'b', &2), FARule::new(&2, 'a', &3),
                 FARule::new(&2, 'b', &3), FARule::new(&3, 'a', &4),
                 FARule::new(&3, 'b', &4)
            ]);

        let nfa_design = NFADesign::new(&1, &to_hashset(&[4]), &rulebook);
        assert!(nfa_design.accept("bab"));
        assert!(nfa_design.accept("bbbbb"));
        assert!(!nfa_design.accept("bbabb"));
    }

    #[test]
    fn test_nfa_freemove() {
        let rulebook = NFARulebook::new(
            vec![FARule::new(&1, '\0',&2), FARule::new(&1, '\0', &4),
                 FARule::new(&2, 'a', &3), FARule::new(&3, 'a', &2),
                 FARule::new(&4, 'a', &5), FARule::new(&5, 'a', &6),
                 FARule::new(&6, 'a', &4)]);
        let result1 = rulebook.next_states(&to_hashset(&[1]), '\0');
        let ans1 = to_hashset(&[2,4]);
        let result2 = rulebook.follow_free_moves(&to_hashset(&[1]));
        let ans2 = to_hashset(&[1,2,4]);
        assert!(result1.is_subset(&ans1) && result1.is_superset(&ans1));
        assert!(result2.is_subset(&ans2) && result2.is_superset(&ans2));

        let nfa_design = NFADesign::new(&1, &to_hashset(&[2, 4]), &rulebook);
        assert!(nfa_design.accept("aa"));
        assert!(nfa_design.accept("aaa"));
        assert!(!nfa_design.accept("aaaaa"));
        assert!(nfa_design.accept("aaaaaa"));
    }
}
