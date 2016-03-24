// Non negative number
pub fn ZERO<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { x }
pub fn ONE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(x) }
pub fn TWO<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(x)) }
pub fn THREE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(x))) }
pub fn FIVE<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(x))))) }
pub fn FIFTEEN<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(x))))))))))))))) }
pub fn HUNDRED<F, T>(p: F, x: T) -> T where F: Fn(T) -> T { p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(p(x)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))) }

