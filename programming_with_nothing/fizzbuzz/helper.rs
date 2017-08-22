use pol::{Pol, Rp};
use std::rc::Rc;
use std::vec::Vec;

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

pub fn to_array(p: &mut Rp) -> Vec<Rp> {
    let _true  = r!(move |x| r!(move |_y| x.clone()));
    let _false = r!(move |_x| r!(move |y| y.clone()));
    let mut out: Vec<Rp> = vec![];
    let left = {
        let _true  = _true.clone();
        r!(move |p: Rp| {
            p.call(_true.clone())
        })
    };
    let right = {
        let _false  = _false.clone();
        r!(move |p: Rp| {
            p.call(_false.clone())
        })
    };
    let is_empty = left.clone();
    let first = {
        let left = left.clone();
        let right = right.clone();
        r!(move |l: Rp| {
            left.call(right.call(l.clone()))
        })
    };
    let rest = {
        let right = right.clone();
        r!(move |l: Rp| {
            right.call(right.clone().call(l.clone()))
        })
    };
    while !to_boolean(&is_empty.call(p.clone())) {
        out.push(first.call(p.clone()));
        *p = rest.call(p.clone());
    }
    out
}
