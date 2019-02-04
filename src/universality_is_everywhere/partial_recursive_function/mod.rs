pub mod prf;
mod nat;

#[cfg(test)]
mod tests {
    use super::prf::{zero};
    use super::nat::*;

    #[test]
    fn test_prf_number() {
        assert!(0 == zero());
        assert!(2 == two());
        assert!(5 == add_three(two()));
    }

    #[test]
    fn test_prf_arithmetic() {
        assert!(5 == add(two(), three()));
        assert!(6 == multiply(two(), three()));
        assert!(1 == decrement(two()));
        assert!(1 == subtract(three(), two()));
        assert!(0 == subtract(two(), three()));
    }

    #[test]
    fn test_prf_divide() {
        assert!(3 == divide(6, 2));
        assert!(3 == divide(10, 3));
        // stack overflow
        // assert!(0 == divide(3, 0));
    }
}
