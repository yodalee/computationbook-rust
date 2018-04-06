pub mod ski;
pub mod skicombinator;
pub mod lambda_to_ski;

use programming_with_nothing::lambda::lambda::{Lambda};

use self::ski::{SKI};
use self::skicombinator::{SKICombinator};
use self::lambda_to_ski::{toSKI, AsFunction};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ski_symbol() {
        let x = SKI::skisymbol("x");
        assert_eq!("x", format!("{}", x));
    }

    #[test]
    fn test_ski_ast() {
        let expr = SKI::skicall(
            SKI::skicall(SKI::s(), SKI::k()),
            SKI::skicall(SKI::i(), SKI::skisymbol("x")));
        assert_eq!("S[K][I[x]]", format!("{}", expr));
    }

    #[test]
    fn test_ski_left_right() {
        let x = SKI::skisymbol("x");
        let y = SKI::skisymbol("y");
        let z = SKI::skisymbol("z");
        let expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::s(), x), y), z);
        let combinator = expr.left().left().left();
        let first_arg = expr.left().left().right();
        let second_arg = expr.left().right();
        let third_arg = expr.right();
        assert_eq!("S", format!("{}", combinator));
        assert_eq!("x", format!("{}", first_arg));
        assert_eq!("y", format!("{}", second_arg));
        assert_eq!("z", format!("{}", third_arg));
        assert_eq!("x[z][y[z]]", format!("{}", combinator.call(vec![first_arg, second_arg, third_arg])));
    }

    #[test]
    fn test_ski_combinator_args() {
        let x = SKI::skisymbol("x");
        let y = SKI::skisymbol("y");
        let z = SKI::skisymbol("z");
        let expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::s(), x), y), z);

        assert_eq!("S[x][y][z]", format!("{}", expr));
        assert_eq!("S", format!("{}", expr.combinator()));
        assert_eq!("x[z][y[z]]", format!("{}", expr.combinator().call(expr.arguments())));
    }

    #[test]
    fn test_ski_callable() {
        let x = SKI::skisymbol("x");
        let y = SKI::skisymbol("y");
        let z = SKI::skisymbol("z");
        let mut expr = SKI::skicall(SKI::skicall(x.clone(), y.clone()), z.clone());
        assert!(!expr.callable(expr.arguments()));
        expr = SKI::skicall(SKI::skicall(SKI::s(), x.clone()), y.clone());
        assert!(!expr.callable(expr.arguments()));
        expr = SKI::skicall(SKI::skicall(SKI::skicall(SKI::s(), x.clone()), y.clone()), z.clone());
        assert!(expr.combinator().callable(expr.arguments()));
    }

    #[test]
    fn test_ski_swap() {
        let swap = SKI::skicall(
            SKI::skicall(
                SKI::s(),
                SKI::skicall(
                    SKI::k(),
                    SKI::skicall(
                        SKI::s(),
                        SKI::i()
                    ),
                ),
            ),
            SKI::k(),
        );
        let mut expr = SKI::skicall(SKI::skicall(swap.clone(), SKI::skisymbol("x")), SKI::skisymbol("y"));

        println!("swap: {}", swap);
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce();
        }
        assert_eq!("y[x]", format!("{}", expr));
    }

    #[test]
    fn test_ski_asfunction() {
        let original = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::i());
        assert_eq!("S[K][I]", format!("{}", original));
        let function = original.as_function_of("x");
        assert_eq!("S[S[K[S]][K[K]]][K[I]]", format!("{}", function));
        assert!(!function.reducible());

        let mut expr = SKI::skicall(function.clone(), SKI::skisymbol("x"));
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce();
        }
        assert_eq!("S[K][I]", format!("{}", expr));
        assert!(format!("{}", original) == format!("{}", expr));
    }
}

pub fn main() {

    /*
    original = SKI::skicall(SKI::skicall(SKI::s(), SKI::skisymbol("x")), SKI::i());
    println!("{}", original);
    function = original.as_function_of("x");
    println!("{}", function);
    expr = SKI::skicall(function.clone(), y.clone());
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("expr reduce: {} != original: {}", expr, original);

    let two = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x")))));
    println!("{} to SKI: {}", two, two.to_ski());
    let (inc, zero) = (SKI::skisymbol("inc"), SKI::skisymbol("zero"));

    expr = SKI::skicall(SKI::skicall(two.to_ski(), inc), zero);
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("{}", expr);

    let identity = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::k());
    println!("{}", identity);
    expr = SKI::skicall(identity, SKI::skisymbol("x"));
    while expr.reducible() {
        println!("{}", expr);
        expr = expr.reduce();
    }
    println!("{}", expr);
    */
}
