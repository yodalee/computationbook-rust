extern crate computationbook;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use computationbook::the_meaning_of_programs::simple::syntax::{Node};
use computationbook::the_meaning_of_programs::simple::machine::{Machine};
use computationbook::the_meaning_of_programs::simple::environment::{Environment};
use pest::Parser;
use pest::iterators::{Pair};
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("simple.pest");

#[derive(Parser)]
#[grammar = "the_meaning_of_programs/simple.pest"]
struct SimpleParser;

pub fn main() {
    for arg in env::args().skip(1) {
        let mut f = File::open(&arg).expect(&format!("file {} not found", arg));
        let mut content = String::new();
        f.read_to_string(&mut content).expect(&format!("Error in reading file {}", arg));
        parse_simple(&content);
    }
}

fn parse_simple(content: &str) {
    let pair = SimpleParser::parse(Rule::simple, content)
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
    let ast = build_stats(pair);
    let mut machine = Machine::new_with_empty_env(ast);
    machine.run();
}

fn build_assign(pair: Pair<Rule>) -> Box<Node> {
    let mut inner = pair.into_inner();
    let lhs = inner.next().unwrap().into_span().as_str();
    let rhs = build_expr(inner.next().unwrap());
    Node::assign(lhs, rhs)
}

fn build_if(pair: Pair<Rule>) -> Box<Node> {
    let mut inner = pair.into_inner();
    let cond = build_expr(inner.next().unwrap());
    let thenstmt = build_stats(inner.next().unwrap());
    match inner.next() {
        Some(stmt) => Node::if_cond_else(cond, thenstmt, build_stats(stmt)),
        None => Node::if_cond_else(cond, thenstmt, Node::donothing())
    }
}

fn build_while(pair: Pair<Rule>) -> Box<Node> {
    let mut inner = pair.into_inner();
    let cond = build_expr(inner.next().unwrap());
    let stmt = build_stats(inner.next().unwrap());
    Node::while_node(cond, stmt)
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
                    Rule::op_lt  => Node::lessthan(lhs, rhs),
                    _ => unreachable!(),
                }
            },
            None => break,
        }
    }
    lhs
}

fn build_stats(pair: Pair<Rule>) -> Box<Node> {
    let inner = pair.into_inner();
    let nodes : Vec<_> = inner.into_iter().map(|pair| build_stat(pair)).collect();
    nodes.iter().rev().fold(Node::donothing(), |acc, node| Node::sequence(node.clone(), acc))
}

fn build_stat(pair: Pair<Rule>) -> Box<Node> {
    match pair.as_rule() {
        Rule::stat_assign => build_assign(pair),
        Rule::stat_if => build_if(pair),
        Rule::stat_while => build_while(pair),
        _ => unreachable!(),
    }
}

fn build_factor(pair: Pair<Rule>) -> Box<Node> {
    match pair.as_rule() {
        Rule::variable => Node::variable(pair.into_span().as_str()),
        Rule::number => Node::number(pair.into_span().as_str().parse::<i64>().unwrap()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpleparser_expr() {
        let pair = SimpleParser::parse(Rule::expr, "1 * 2 + 3 * 4")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
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

    #[test]
    fn test_simpleparser_assign() {
        let pair = SimpleParser::parse(Rule::stat, "x = 42;")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        match pair.as_rule() {
            Rule::stat_assign => {
                let ast = build_assign(pair);
                let mut env = Environment::new();
                env.add("x", Node::number(0));
                let mut machine = Machine::new(ast, env);
                machine.run();
                assert_eq!(42, machine.environment.get("x").value());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_simpleparser_if() {
        let pair = SimpleParser::parse(Rule::stat, "if (x) { y = 1; } else { y = 2; }")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        match pair.as_rule() {
            Rule::stat_if => {
                let ast = build_if(pair);
                let mut env = Environment::new();
                env.add("x", Node::boolean(true));

                let mut machine = Machine::new(ast, env);
                machine.run();
                assert_eq!(1, machine.environment.get("y").value());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_simpleparser_ifelse() {
        let pair = SimpleParser::parse(Rule::stat, "if (x) { y = 1; }")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        match pair.as_rule() {
            Rule::stat_if => {
                let ast = build_if(pair);
                let mut env = Environment::new();
                env.add("x", Node::boolean(false));
                env.add("y", Node::number(10));

                let mut machine = Machine::new(ast, env);
                machine.run();
                assert_eq!(10, machine.environment.get("y").value());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_simpleparser_while() {
        let pair = SimpleParser::parse(Rule::stat, "while (x < 5) { x = x * 3; }")
                     .unwrap_or_else(|e| panic!("{}", e))
                     .next().unwrap();
        match pair.as_rule() {
            Rule::stat_while => {
                let ast = build_while(pair);
                let mut env = Environment::new();
                env.add("x", Node::number(1));

                let mut machine = Machine::new(ast, env);
                machine.run();
                assert_eq!(9, machine.environment.get("x").value());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_simpleparser_stats() {
        let pair = SimpleParser::parse(Rule::stats, "x = 1; while \n (x < 5) { x = x * 3; };")
                     .unwrap_or_else(|e| panic!("{}", e))
                     .next().unwrap();
        match pair.as_rule() {
            Rule::stats => {
                let ast = build_stats(pair);
                let mut env = Environment::new();

                let mut machine = Machine::new(ast, env);
                machine.run();
                assert_eq!(9, machine.environment.get("x").value());
            }
            _ => unreachable!(),
        }
    }
}

