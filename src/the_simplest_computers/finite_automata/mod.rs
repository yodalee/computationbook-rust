pub mod farule;
pub mod dfarulebook;
pub mod dfa;
pub mod dfadesign;
pub mod nfarulebook;
pub mod nfa;
pub mod nfadesign;
pub mod nfasimulation;
pub mod stateset;

#[cfg(test)]
mod tests {
    use super::farule::*;
    use super::dfarulebook::*;
    use super::dfa::*;
    use super::dfadesign::*;
    use super::nfarulebook::*;
    use super::nfa::*;
    use super::nfadesign::*;
    use super::nfasimulation::*;
    use super::stateset::*;
    use helper::*;

    use std::collections::HashSet;

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
        assert!(DFA::new(1, &vec![1, 3], &rulebook).accepting());
        assert!(!DFA::new(1, &vec![3], &rulebook).accepting());

        let mut dfa = DFA::new(1, &vec![3], &rulebook);
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

        dfa = DFA::new(1, &vec![3], &rulebook);
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
        let dfa_design = DFADesign::new(1, &vec![3], &rulebook);
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
        assert!(hashset_eq(&result1, &ans1));
        assert!(hashset_eq(&result2, &ans2));
        assert!(hashset_eq(&result3, &ans3));
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
        assert!(hashset_eq(&result1, &ans1));
        assert!(hashset_eq(&result2, &ans2));

        let nfa_design = NFADesign::new(&1, &to_hashset(&[2, 4]), &rulebook);
        assert!(nfa_design.accept("aa"));
        assert!(nfa_design.accept("aaa"));
        assert!(!nfa_design.accept("aaaaa"));
        assert!(nfa_design.accept("aaaaaa"));
    }

    #[test]
    fn test_nfa_start_state() {
        let rulebook = NFARulebook::new(vec![
            FARule::new(&1, 'a',  &1), FARule::new(&1, 'a',  &2),
            FARule::new(&1, '\0', &2), FARule::new(&2, 'b',  &3),
            FARule::new(&3, 'b',  &1), FARule::new(&3, '\0', &2)
        ]);
        let nfa_design = NFADesign::new(&1, &to_hashset(&[3]), &rulebook);
        let result1 = nfa_design.to_nfa().current_state();
        let ans1 = to_hashset(&[1, 2]);
        let result2 = nfa_design.to_nfa_with_state(&to_hashset(&[2])).current_state();
        let ans2 = to_hashset(&[2]);
        let result3 = nfa_design.to_nfa_with_state(&to_hashset(&[3])).current_state();
        let ans3 = to_hashset(&[3, 2]);
        assert!(hashset_eq(&result1, &ans1));
        assert!(hashset_eq(&result2, &ans2));
        assert!(hashset_eq(&result3, &ans3));

        let mut nfa = nfa_design.to_nfa_with_state(&to_hashset(&[2,3]));
        nfa.read_character('b');
        assert!(hashset_eq(&nfa.current_state(), &to_hashset(&[1,2,3])));
    }

    #[test]
    fn test_nfa_simulation() {
        let rulebook = NFARulebook::new(vec![
            FARule::new(&1, 'a',  &1), FARule::new(&1, 'a',  &2),
            FARule::new(&1, '\0', &2), FARule::new(&2, 'b',  &3),
            FARule::new(&3, 'b',  &1), FARule::new(&3, '\0', &2)
        ]);
        let nfa_design = NFADesign::new(&1, &to_hashset(&[3]), &rulebook);
        let nfa_simulation = NFASimulation::new(&nfa_design);
        let result1 = nfa_simulation.next_state(&to_hashset(&[1,2]), 'a');
        let result2 = nfa_simulation.next_state(&to_hashset(&[1,2]), 'b');
        let result3 = nfa_simulation.next_state(&to_hashset(&[2,3]), 'b');
        let result4 = nfa_simulation.next_state(&to_hashset(&[1,2,3]), 'b');
        let result5 = nfa_simulation.next_state(&to_hashset(&[1,2,3]), 'a');
        assert!(hashset_eq(&result1, &to_hashset(&[1,2])));
        assert!(hashset_eq(&result2, &to_hashset(&[2,3])));
        assert!(hashset_eq(&result3, &to_hashset(&[1,2,3])));
        assert!(hashset_eq(&result4, &to_hashset(&[1,2,3])));
        assert!(hashset_eq(&result5, &to_hashset(&[1,2])));
    }

    #[test]
    fn test_nfa_simulation_rule_for() {
        let rulebook = NFARulebook::new(vec![
            FARule::new(&1, 'a',  &1), FARule::new(&1, 'a',  &2),
            FARule::new(&1, '\0', &2), FARule::new(&2, 'b',  &3),
            FARule::new(&3, 'b',  &1), FARule::new(&3, '\0', &2)
        ]);
        let alphabet = rulebook.alphabet();
        assert_eq!(&alphabet, &['a','b']);

        let nfa_design = NFADesign::new(&1, &to_hashset(&[3]), &rulebook);
        let nfa_simulation = NFASimulation::new(&nfa_design);
        let result1 = nfa_simulation.rule_for(&to_hashset(&[1,2]));
        let ans1 = vec![
            FARule::new(&to_hashset(&[1,2]), 'a', &to_hashset(&[1,2])),
            FARule::new(&to_hashset(&[1,2]), 'b', &to_hashset(&[2,3]))
        ];
        let result2 = nfa_simulation.rule_for(&to_hashset(&[2,3]));
        let ans2 = vec![
            FARule::new(&to_hashset(&[2,3]), 'a', &to_hashset(&[])),
            FARule::new(&to_hashset(&[2,3]), 'b', &to_hashset(&[1,2,3]))
        ];
        assert!(result1.into_iter().zip(ans1.into_iter())
            .all(|(ref rule1, ref rule2)|
                 rule1.character == rule2.character &&
                 hashset_eq(&rule1.state.0, &rule2.state) &&
                 hashset_eq(&rule1.next_state.0, &rule2.next_state)));
        assert!(result2.iter().zip(ans2.iter())
            .all(|(ref rule1, ref rule2)|
                 rule1.character == rule2.character &&
                 hashset_eq(&rule1.state.0, &rule2.state) &&
                 hashset_eq(&rule1.next_state.0, &rule2.next_state)));
    }

    #[test]
    fn test_nfa_simulation_discover() {
        let rulebook = NFARulebook::new(vec![
            FARule::new(&1, 'a',  &1), FARule::new(&1, 'a',  &2),
            FARule::new(&1, '\0', &2), FARule::new(&2, 'b',  &3),
            FARule::new(&3, 'b',  &1), FARule::new(&3, '\0', &2)
        ]);
        let nfa_design = NFADesign::new(&1, &to_hashset(&[3]), &rulebook);
        let nfa_simulation = NFASimulation::new(&nfa_design);

        let start_state = nfa_design.to_nfa().current_state();
        println!("{:?}", start_state);
        // let startset = StateSet::new(&start_state);
        let mut stateset = HashSet::new();
        stateset.insert(StateSet::new(&start_state));

        let (_states, _rules) = nfa_simulation.discover_states_and_rules(&mut stateset);
        assert!(!nfa_design.to_nfa_with_state(&to_hashset(&[1,2])).accepting());
        assert!(nfa_design.to_nfa_with_state(&to_hashset(&[2,3])).accepting());
    }

    #[test]
    fn test_simulation_nfa_to_dfa() {
        let rulebook = NFARulebook::new(vec![
            FARule::new(&1, 'a',  &1), FARule::new(&1, 'a',  &2),
            FARule::new(&1, '\0', &2), FARule::new(&2, 'b',  &3),
            FARule::new(&3, 'b',  &1), FARule::new(&3, '\0', &2)
        ]);
        let nfa_design = NFADesign::new(&1, &to_hashset(&[3]), &rulebook);
        let nfa_simulation = NFASimulation::new(&nfa_design);
        let dfa_design = nfa_simulation.to_dfa_design();
        assert!(!dfa_design.accept("aaa"));
        assert!(dfa_design.accept("aab"));
        assert!(dfa_design.accept("bbbabb"));
    }
}
