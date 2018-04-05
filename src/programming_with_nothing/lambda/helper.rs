use super::pol::{Pol, Rp};

use std::rc::Rc;

pub fn to_integer(p: &Rp) -> i32 {
    // p(|n|{n+1})(0)
    let np1 = r!(|n: Rp| {
        let np1 = {
            let n = match n.as_ref() {
                &Pol::I(n) => n,
                _ => panic!(),
            };
            n + 1
        };
        Rc::new(Pol::I(np1))
    });

    let ans = p.call(np1).call(Rc::new(Pol::I(0)));
    match ans.as_ref() {
        &Pol::I(n) => n,
        _ => panic!(),
    }
}

pub fn to_boolean(p: &Rp) -> bool {
    // p(true)(false)
    let ans = p.call(Rc::new(Pol::B(true))).call(Rc::new(Pol::B(false)));
    match ans.as_ref() {
        &Pol::B(n) => n,
        _ => panic!(),
    }
}

static CHARMAP: &'static str = "0123456789BFiuz";
pub fn to_char(c: &Rp) -> char {
    CHARMAP.chars().nth(to_integer(&c) as usize).unwrap()
}
