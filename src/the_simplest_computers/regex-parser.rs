extern crate computationbook;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use computationbook::the_simplest_computers::regular_expressions::regex::{Regex};
use computationbook::the_simplest_computers::regular_expressions::tonfa::{ToNFA};
use pest::Parser;
use pest::iterators::{Pair};
use std::env;

#[cfg(debug_assertions)]
const _GRAMMER: &'static str = include_str!("regex.pest");

#[derive(Parser)]
#[grammar = "the_simplest_computers/regex.pest"]
struct RegexParser;

pub fn main() {
    let args : Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: regex-parser \"pattern\" string");
        eprintln!("Add double quote to prevent shell expand | and *");
    }

    let pair = RegexParser::parse(Rule::choose, &args[1])
                .unwrap_or_else(|e| panic!("{}", e))
                .next().unwrap();
    let pattern = build_regex(pair);
    if pattern.matches(&args[2]) {
        println!("Pattern {} can matches {}", args[1], args[2]);
    } else {
        println!("Pattern {} cannot matches {}", args[1], args[2]);
    }
}

fn build_regex(pair: Pair<Rule>) -> Box<Regex> {
    match pair.as_rule() {
        Rule::empty => Regex::empty(),
        Rule::alpha => Regex::literal(pair
            .into_span().as_str().chars().next().unwrap()),
        Rule::repeat => {
            let mut inner = pair.into_inner();
            let regex = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(_) => Regex::repeat(regex),
                None => regex,
            }
        },
        Rule::choose => {
            let mut inner = pair.into_inner();
            let fst = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(rest) => {
                    Regex::choose(fst, build_regex(rest))
                },
                None => fst,
            }
        },
        Rule::concat => {
            let mut inner = pair.into_inner();
            let fst = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(rest) => Regex::concatenate(fst, build_regex(rest)),
                None => fst,
            }
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexparser_empty() {
        let pair = RegexParser::parse(Rule::choose, "")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(!pattern.matches("a"));
    }

    #[test]
    fn test_regexparser_literal() {
        let pair = RegexParser::parse(Rule::choose, "a")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("a"));
    }

    #[test]
    fn test_regexparser_repeat() {
        let pair = RegexParser::parse(Rule::choose, "a*")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(pattern.matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }

    #[test]
    fn test_regexparser_choose() {
        let pair = RegexParser::parse(Rule::choose, "a|b")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(pattern.matches("b"));
        assert!(!pattern.matches("ab"));
    }

    #[test]
    fn test_regexparser_concat() {
        let pair = RegexParser::parse(Rule::choose, "abcd")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(!pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(pattern.matches("abcd"));
        assert!(!pattern.matches("abcdefg"));
    }
}
