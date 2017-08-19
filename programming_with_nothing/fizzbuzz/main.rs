#[macro_use]
mod pol;
mod helper;

use pol::{Pol, Rp};
use helper::*;
use std::rc::Rc;

fn main() {
    // ************************************
    // Non-Negative Number

    // number
    // |_p| {|x| { x } } or |p| { |x| { p(p(...p(x)...) } }
    let zero  = r!(|_p| r!(|x| x));
    let one   = r!(|p: Rp| r!(move |x| p.call(x)));
    let two   = r!(|p: Rp| r!(move |x| p.call(p.call(x))));
    let three = r!(|p: Rp| r!(move |x| p.call(p.call(p.call(x)))));
    let five  = r!(|p: Rp| r!(move |x| p.call(p.call(p.call(p.call(p.call(x)))))));
    let fifteen  = r!(|p: Rp| r!(move |x| p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(x)))))))))))))))));
    let hundred  = r!(|p: Rp| r!(move |x| p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(
                                        p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(p.call(x
    ))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))));

    // increment
    // |n| { |p| { |x| { p(n(p)(x)) }  } }
    let incr = r!(|n: Rp| {
        r!(move |p: Rp| {
            let n = n.clone();
            let p = p.clone();
            r!(move |x| p.call(n.call(p.clone()).call(x)))
        })
    });

    // add
    // |m| { |n| { n(incr)(m) } }
    let add = r!(move |m: Rp| {
        let incr = incr.clone();
        r!(move |n: Rp| n.call(incr.clone()).call(m.clone()))
    });

    println!("NUMBER 0:{} 1:{} 2:{} 3:{} 5:{} 15:{} 100:{}",
             to_integer(zero.clone()), to_integer(one.clone()),
             to_integer(two.clone()), to_integer(three.clone()),
             to_integer(five.clone()), to_integer(fifteen.clone()),
             to_integer(hundred.clone()));

    // ************************************
    // Boolean

    // true
    // |x| { |y| { x }}
    let _true  = r!(move |x| r!(move |_y| x.clone()));
    // false
    // |x| { |y| { y }}
    let _false = r!(move |_x| r!(move |y| y.clone()));
    // if
    // |b| { b }
    let _if = r!(|b: Rp| b.clone());

    println!("BOOLEAN true:{} false:{}",
             to_boolean(_true.clone()), to_boolean(_false.clone()));
    println!("BOOLEAN true:{}", to_integer(_true.call(one.clone()).call(zero.clone())));
}
