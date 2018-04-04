pub mod regex;
pub mod tonfa;
mod nfa;
mod helper;

#[cfg(test)]
mod tests {
    use super::regex::*;
    use super::tonfa::*;


    #[test]
    fn test_regex_pattern() {
        let pattern = Regex::repeat(Regex::choose(Regex::concatenate(Regex::literal('a'), Regex::literal('b')), Regex::literal('a')));
        assert_eq!("(ab|a)*", format!("{}", pattern));
    }

    #[test]
    fn test_regex_empty() {
        let pattern = Regex::empty();
        println!("Regex '{}'", pattern);
        assert!(pattern.matches(""));
        assert!(!pattern.matches("a"));
    }

    #[test]
    fn test_regex_literal() {
        let pattern = Regex::literal('a');
        println!("Regex '{}'", pattern);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("b"));
    }

    #[test]
    fn test_regex_concatenate() {
        let pattern = Regex::concatenate(Regex::literal('a'), Regex::literal('b'));
        println!("Regex '{}'", pattern);
        assert!(!pattern.matches("a"));
        assert!(pattern.matches("ab"));
        assert!(!pattern.matches("abc"));
    }

    #[test]
    fn test_regex_choose() {
        let pattern = Regex::choose(Regex::literal('a'), Regex::literal('b'));
        println!("Regex '{}'", pattern);
        assert!(pattern.matches("a"));
        assert!(pattern.matches("b"));
        assert!(!pattern.matches("c"));
    }

    #[test]
    fn test_regex_repeat() {
        let pattern = Regex::repeat(Regex::literal('a'));
        println!("Regex '{}'", pattern);
        assert!(pattern.matches("a"));
        assert!(pattern.matches("aaaa"));
        assert!(!pattern.matches("b"));
    }

    #[test]
    fn test_regex_complex() {
        let pattern = Regex::repeat(Regex::concatenate(Regex::literal('a'), Regex::choose(Regex::empty(), Regex::literal('b'))));
        println!("Regex '{}'", pattern);
        assert!(pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(pattern.matches("ab"));
        assert!(pattern.matches("aba"));
        assert!(pattern.matches("abab"));
        assert!(pattern.matches("abaab"));
        assert!(!pattern.matches("abba"));
    }
}
