use std::ops::Mul;
use std::ops::Add;

pub mod sign;
pub mod numeric;

fn calculate<T: Mul<T, Output=T>+Copy>(x: T, y: T, z: T) -> T {
    (x*y) * (x*z)
}

fn sum_of_squares<T: Mul<T, Output=T>+Add<T, Output=T>+Copy>(x: T, y: T) -> T {
    (x*x) + (y*y)
}

#[cfg(test)]
mod tests {
    use super::sign::{Sign};
    use super::numeric::{HasSign};

    #[test]
    fn test_sign_mul() {
        assert_eq!(Sign::POSITIVE, Sign::POSITIVE * Sign::POSITIVE);
        assert_eq!(Sign::ZERO, Sign::NEGATIVE * Sign::ZERO);
        assert_eq!(Sign::NEGATIVE, Sign::POSITIVE * Sign::NEGATIVE);
        assert_eq!(Sign::POSITIVE, 6i32.sign());
        assert_eq!(Sign::NEGATIVE, (-9i32).sign());
        assert_eq!(Sign::NEGATIVE, 6i32.sign() * -9i32.sign());
    }

    #[test]
    fn test_sign_calculate() {
        assert_eq!(0, calculate(3, -5, 0));
        assert_eq!(Sign::ZERO, calculate(Sign::POSITIVE, Sign::NEGATIVE, Sign::ZERO));
        assert!((6 * -9).sign() == (6.sign() * -9.sign()));
        assert!((100 * 0).sign() == (100.sign() * 0.sign()));
        assert!(calculate(1, -2, -3).sign() == calculate(1.sign(), -2.sign(), -3.sign()));
    }

    #[test]
    fn test_sign_add() {
        assert_eq!(Sign::POSITIVE, Sign::POSITIVE + Sign::POSITIVE);
        assert_eq!(Sign::NEGATIVE, Sign::NEGATIVE + Sign::ZERO);
        assert_eq!(Sign::UNKNOWN, Sign::POSITIVE + Sign::NEGATIVE);
        assert_eq!(Sign::POSITIVE, (Sign::POSITIVE + Sign::NEGATIVE) * Sign::ZERO + Sign::POSITIVE);
        assert!((10 + 3).sign() == (10.sign() + 3.sign()));
        assert!((-5 + 0).sign() == (-5.sign() + 0.sign()));
        assert!(!((6 + -9).sign() == (6.sign() + -9.sign())));
        assert_eq!(Sign::NEGATIVE, (6 + -9).sign());
        assert_eq!(Sign::UNKNOWN, 6.sign() + -9.sign());
    }

    #[test]
    fn test_lessthan() {
        assert!(Sign::POSITIVE <= Sign::POSITIVE);
        assert!(!(Sign::POSITIVE <= Sign::UNKNOWN));
        assert!(!(Sign::POSITIVE <= Sign::NEGATIVE));
        assert!((6 * -9).sign() <= (6.sign() * -9.sign()));
        assert!((-5 + 0).sign() <= (-5.sign() + 0.sign()));
        assert!(!((6 + -9).sign() <= (6.sign() + -9.sign())));
    }

    #[test]
    fn test_sum_of_square() {
        let input = [Sign::POSITIVE, Sign::ZERO, Sign::NEGATIVE];
        let mut output: Vec<Sign> = Vec::new();
        for i in input.iter() {
            for j in input.iter() {
                output.push(sum_of_squares(*i, *j))
            }
        }
        assert_eq!(output, &[Sign::POSITIVE, Sign::POSITIVE, Sign::POSITIVE,
                             Sign::POSITIVE, Sign::ZERO, Sign::POSITIVE,
                             Sign::POSITIVE, Sign::POSITIVE, Sign::POSITIVE]);
    }
}
