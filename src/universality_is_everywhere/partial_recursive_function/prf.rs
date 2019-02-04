use std::rc::{Rc};

type F = Rc<Fn(&mut[u32]) -> u32>;
type G = Rc<Fn(&mut[u32], &mut u32, &mut u32) -> u32>;
type M = Rc<Fn(u32) -> u32>;

pub fn zero_arg(_x: &mut[u32]) -> u32 {
    0
}

pub fn zero() -> u32 {
    zero_arg(&mut[])
}

pub fn increment(n: u32) -> u32 {
    n+1
}

pub fn recurse(f: F, g: G, values: &mut[u32]) -> u32 {
    let easier_values = &mut values.to_vec()[..];
    if let Some((last, elems)) = easier_values.split_last_mut() {
        if *last == 0 {
            return f(elems);
        } else {
            *last = *last - 1;
        }
    }
    let mut easier_result = recurse(f.clone(), g.clone(), easier_values);
    if let Some((last, elems)) = easier_values.split_last_mut() {
        return g(elems, last, &mut easier_result);
    }
    unreachable!();
}

pub fn minimize(f: M) -> u32 {
    let mut n = 0u32;
    loop {
        if f(n) == 0 {
            break;
        }
        n = n + 1;
    }
    n
}
