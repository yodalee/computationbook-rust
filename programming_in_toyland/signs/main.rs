use std::ops::Mul;
use std::ops::Add;

mod sign;
mod numeric;

use sign::{Sign};
use numeric::{hasSign};

fn calculate<T: Mul<T, Output=T>+Copy>(x: T, y: T, z: T) -> T {
    (x*y) * (x*z)
}

fn sum_of_squares<T: Mul<T, Output=T>+Add<T, Output=T>+Copy>(x: T, y: T) -> T {
    (x*x) + (y*y)
}

pub fn main() {
    println!("{}", Sign::POSITIVE * Sign::POSITIVE);
    println!("{}", Sign::NEGATIVE * Sign::ZERO);
    println!("{}", Sign::POSITIVE * Sign::NEGATIVE);
    println!("{}", 6i32.sign());
    println!("{}", (-9i32).sign());
    println!("{}", 6i32.sign() * -9i32.sign());
    println!("cal(3, -5, 0): {}", calculate(3, -5, 0));
    println!("cal(POSITIVE, NEGATIVE, ZERO): {}", calculate(Sign::POSITIVE, Sign::NEGATIVE, Sign::ZERO));
    println!("{}", (6 * -9).sign() == (6.sign() * -9.sign()));
    println!("{}", (100 * 0).sign() == (100.sign() * 0.sign()));
    println!("{}", calculate(1, -2, -3).sign() == calculate(1.sign(), -2.sign(), -3.sign()));
    println!("{}", Sign::POSITIVE + Sign::POSITIVE);
    println!("{}", Sign::NEGATIVE + Sign::ZERO);
    println!("{}", Sign::POSITIVE + Sign::NEGATIVE);
    println!("{}", (Sign::POSITIVE + Sign::NEGATIVE) * Sign::ZERO + Sign::POSITIVE);
    println!("{}", (10 + 3).sign() == (10.sign() + 3.sign()));
    println!("{}", (-5 + 0).sign() == (-5.sign() + 0.sign()));
    println!("{}", (6 + -9).sign() == (6.sign() + -9.sign()));
    println!("{}", (6 + -9).sign());
    println!("{}", 6.sign() + -9.sign());
    println!("{}", Sign::POSITIVE <= Sign::POSITIVE);
    println!("{}", Sign::POSITIVE <= Sign::UNKNOWN);
    println!("{}", Sign::POSITIVE <= Sign::NEGATIVE);
    println!("{}", (6 * -9).sign() <= (6.sign() * -9.sign()));
    println!("{}", (-5 + 0).sign() <= (-5.sign() + 0.sign()));
    println!("{}", (6 + -9).sign() <= (6.sign() + -9.sign()));
    let input = [Sign::POSITIVE, Sign::ZERO, Sign::NEGATIVE];
    let mut output: Vec<Sign> = Vec::new();
    for i in input.iter() {
        for j in input.iter() {
            output.push(sum_of_squares(*i, *j))
        }
    }
    println!("[{}]", output.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(","));
}
