use std::ops::Add;

pub fn INCR<T: Add<i32, Output=T>>(x: T) -> T { x+1 }

pub fn to_integer<F, T>(p: F) -> i32
    where F: Fn(fn(T) -> T, i32) -> i32,
          T: Add<i32, Output=T> { p(INCR, 0) }


