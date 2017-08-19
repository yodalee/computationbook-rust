use std::rc::Rc;

type Rp = Rc<Pol>;
enum Pol {
    C(Rc<Fn(Rp) -> Rp>),
    I(i32),
}
use Pol::{C, I};
macro_rules! r {
    ($cl:expr) => {Rc::new(C(Rc::new($cl)))}
}
impl Pol {
    fn call(&self, x: Rp) -> Rp {
        match self {
            &C(ref c) => c(x),
            _ => panic!(),
        }
    }
}

fn to_integer(p: Rp) -> i32 {
    // p(|n|{n+1})(0)
    let np1 = r!(|n: Rp| {
        let np1 = {
            let n = match n.as_ref() {
                &I(n) => n,
                _ => panic!(),
            };
            n + 1
        };
        Rc::new(I(np1))
    });
    let ans = p.call(np1).call(Rc::new(I(0)));
    match ans.as_ref() {
        &I(n) => n,
        _ => panic!(),
    }
}

fn main() {
    // |_p| {|x| { x } }
    let zero = r!(|_p| r!(|x| x));
    println!("{}", to_integer(zero.clone()));

    // |p| { |x| { p(x) } }
    let one = r!(|p: Rp| r!(move |x| p.call(x)));
    println!("{}", to_integer(one.clone()));

    // |p| { |x| { p(p(x)) } }
    let two = r!(|p: Rp| r!(move |x| p.call(p.call(x))));
    println!("{}", to_integer(two.clone()));

    // |n| { |p| { |x| { p(n(p)(x)) }  } }
    let increment = r!(|n: Rp| {
        let n = n.clone();
        r!(move |p: Rp| {
            let n = n.clone();
            let p = p.clone();
            r!(move |x| p.call(n.call(p.clone()).call(x)))
        })
    });

    // |m| { |n| { n(increment)(m) } }
    let increment_for_add = increment.clone();
    let add = r!(move |m: Rp| {
        let increment = increment_for_add.clone();
        let m = m.clone();
        r!(move |n: Rp| n.call(increment.clone()).call(m.clone()))
    });

    // add(one)(two)
    let a = add.clone().call(one.clone()).call(two.clone());
    println!("1+2={}", to_integer(a));
}
