extern crate computationbook;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use computationbook::programming_with_nothing::lambda::lambda::Lambda;
use computationbook::programming_with_nothing::lambda::reduce::Reduce;
use pest::Parser;
use pest::iterators::{Pair};
use std::env;

#[cfg(debug_assertions)]
const _GRAMMER: &'static str = include_str!("lambda.pest");

#[derive(Parser)]
#[grammar = "programming_with_nothing/lambda.pest"]
struct LambdaParser;

pub fn main() {
    let args : Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lambda-parser lambda_string");
    }

    let pair = LambdaParser::parse(Rule::expression, &args[1])
                .unwrap_or_else(|e| panic!("{}", e))
                .next().unwrap();
    let expr = build_lambda(pair);
    println!("Expression {}", expr);
    println!("Reduce {}", expr.reduce());
}

fn build_lambda(pair: Pair<Rule>) -> Box<Lambda> {
    match pair.as_rule() {
        Rule::call => {
            let mut inner = pair.into_inner();
            let fst = build_lambda(inner.next().unwrap());
            let args : Vec<_> = inner.into_iter().map(|pair| build_lambda(pair)).collect();
            args.iter().fold(fst, |l, r| Lambda::lccall(l, r.clone()))
        }
        Rule::variable => Lambda::lcvar(pair.into_span().as_str()),
        Rule::function => {
            let mut inner = pair.into_inner();
            let parameter = inner.next().unwrap().into_span().as_str();
            let body = build_lambda(inner.next().unwrap());
            Lambda::lcfun(parameter, body)
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambdaparser() {
        let pair = LambdaParser::parse(Rule::expression,
                                       "-> x { x[x] }[-> y { y }]")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let mut expr = build_lambda(pair);
        println!("expression: {}", expr);
        expr = expr.reduce();
        println!("reduce: {}", expr);
        assert_eq!("-> y { y }[-> y { y }]",
                   format!("{}", expr));
    }
}
