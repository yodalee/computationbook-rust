extern crate computationbook;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use computationbook::the_meaning_of_programs::simple::syntax::Node;
use computationbook::the_meaning_of_programs::simple::machine::*;
use pest::Parser;
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "the_meaning_of_programs/simple.pest"]
struct SimpleParser;

static PARSE_STR: &str = "1 * 2 + 3 * 4";

pub fn main() {
    let pairs = SimpleParser::parse(Rule::expr, PARSE_STR).unwrap_or_else(|e| panic!("{}", e));
    for pair in pairs {
        println!("Rule: {:?}", pair.as_rule());
        match pair.as_rule() {
            Rule::expr => {
                let ast = build_expr(pair);
                let mut machine = Machine::new_with_empty_env(ast);
                machine.run();
                // Notice we have not deal with precedence yet
                assert_eq!(20, machine.get_expression().value());
            },
            _ => unreachable!(),
        };
    }
}

fn build_expr(pair: Pair<Rule>) -> Box<Node> {
    let mut inner = pair.into_inner();
    let mut lhs = build_factor(inner.next().unwrap());
    loop {
        let op = inner.next();
        match op {
            Some(op) => {
                let rhs = build_factor(inner.next().unwrap());
                lhs = match op.as_rule() {
                    Rule::op_add => Node::add(lhs, rhs),
                    Rule::op_mul => Node::multiply(lhs, rhs),
                    _ => unreachable!(),
                }
            },
            None => break,
        }
    }
    lhs
}

fn build_factor(pair: Pair<Rule>) -> Box<Node> {
    match pair.as_rule() {
        Rule::variable => Node::variable(pair.into_span().as_str()),
        Rule::number => Node::number(pair.into_span().as_str().parse::<i64>().unwrap()),
        _ => unreachable!(),
    }
}
