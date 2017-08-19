use pol::{Pol, Rp};
use std::rc::Rc;

pub fn to_integer(p: Rp) -> i32 {
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

