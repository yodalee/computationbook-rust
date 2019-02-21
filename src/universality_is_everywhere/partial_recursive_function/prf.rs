use std::rc::{Rc};

pub type F = Rc<Fn(&[u32]) -> u32>;
pub type G = Rc<Fn(&[u32], &u32, &u32) -> u32>;
pub type M = Rc<Fn(&u32) -> u32>;

pub fn zero() -> u32 {
    0
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
            *last -= 1;
        }
    }
    let mut easier_result = recurse(f.clone(), g.clone(), easier_values);
    let (last, elems) = easier_values.split_last_mut().unwrap();
    return g(elems, last, &mut easier_result);
}

pub fn minimize(f: M) -> u32 {
    let mut n = 0u32;
    loop {
        if f(&n) == 0 {
            break;
        }
        n += 1;
    }
    n
}
