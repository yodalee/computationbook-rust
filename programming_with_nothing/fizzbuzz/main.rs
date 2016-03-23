use std::ops::Add;

// Non negative number
fn ZERO<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { x }
fn ONE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(x) }
fn TWO<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(x)) }
fn THREE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(x))) }
fn FIVE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(x))))) }
fn FIFTEEN<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(x))))))))))))))) }
fn HUNDRED<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(x)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))) }

fn INCR<T: Add<i32, Output=T>>(x: T) -> T { x+1 }

fn to_integer<F, T>(p: F) -> i32
    where F: Fn(fn(T) -> T, i32) -> i32,
          T: Add<i32, Output=T> { p(INCR, 0) }

pub fn main() {
    println!("{}", to_integer(ZERO));
    println!("{}", to_integer(ONE));
    println!("{}", to_integer(THREE));
    println!("{}", to_integer(FIVE));
    println!("{}", to_integer(FIFTEEN));
    println!("{}", to_integer(HUNDRED));
}
