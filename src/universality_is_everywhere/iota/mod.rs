pub mod ski;
pub mod iota;
pub mod lambda_to_ski;

use self::ski::{SKI};
use self::iota::{s, k, i, iota};
use self::lambda_to_ski::{ToSKI, AsFunction};

#[cfg(test)]
mod tests {
    use super::*;
    use programming_with_nothing::lambda::lambda::{Lambda};

    #[test]
    fn test_iota_s() {
        let mut expr = s().to_iota();
        println!("{}", expr);
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce()
        }
        println!("{}", expr);
        assert_eq!("S", format!("{}", expr));
    }

    #[test]
    fn test_iota_k() {
        let mut expr = k().to_iota();
        println!("{}", expr);
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce()
        }
        println!("{}", expr);
        assert_eq!("K", format!("{}", expr));
    }

    #[test]
    fn test_iota_i() {
        let mut expr = i().to_iota();
        println!("{}", expr);
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce()
        }
        println!("{}", expr);
        assert_eq!("S[K][K[K]]", format!("{}", expr));
    }

    fn test_iota_verify_i() {
        let identity = SKI::skicall(SKI::skicall(SKI::s(), SKI::k()), SKI::skicall(SKI::k(), SKI::k()));
        let mut expr = SKI::skicall(identity, SKI::skisymbol("x"));
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce()
        }
        println!("{}", expr);
        assert_eq!("x", format!("{}", expr));
    }

    #[test]
    fn test_iota_two() {
        let two = Lambda::lcfun("p", Lambda::lcfun("x", Lambda::lccall(Lambda::lcvar("p"), Lambda::lccall(Lambda::lcvar("p"), Lambda::lcvar("x")))));
        let mut expr = (*two.to_ski()).to_iota();
        expr = SKI::skicall(SKI::skicall(expr, SKI::skisymbol("inc")), SKI::skisymbol("zero"));
        while expr.reducible() {
            println!("{}", expr);
            expr = expr.reduce()
        }
        println!("{}", expr);
        assert_eq!("inc[inc[zero]]", format!("{}", expr));
    }
}
