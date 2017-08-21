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

    println!("NUMBER 0:{} 1:{} 2:{} 3:{} 5:{} 15:{} 100:{}",
             to_integer(&zero), to_integer(&one), to_integer(&two),
             to_integer(&three), to_integer(&five), to_integer(&fifteen),
             to_integer(&hundred));

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
             to_boolean(&_true), to_boolean(&_false));
    println!("BOOLEAN (true)?1:0 :{}", to_integer(&_true.call(one.clone()).call(zero.clone())));

    // ************************************
    // Predicate

    // isZero
    // |n| { n( |x| {FALSE})(TRUE) }
    let isZero = {
        let _false = _false.clone();
        let _true  = _true.clone();
        let retFalse = r!(move |_x: Rp| _false.clone());
        r!(move |n: Rp| {
            n.call(retFalse.clone()).call(_true.clone())
        })
    };

    println!("IS_ZERO zero:{} three:{}",
             to_boolean(&isZero.call(zero.clone())),
             to_boolean(&isZero.call(three.clone())));

    // ************************************
    // Pair

    // pair
    // |x| { |y| { |f| { f(x)(y) }}}
    let pair = r!(move |x: Rp| {
        r!(move |y: Rp| {
            let x = x.clone();
            let y = y.clone();
            r!(move |f: Rp| { f.call(x.clone()).call(y.clone()) })
        })
    });

    // left
    // |p| { p(|x| { |y| { x }} ) }
    let left = {
        let _true  = _true.clone();
        r!(move |p: Rp| {
            p.call(_true.clone())
        })
    };
    // right
    // |p| { p(|x| { |y| { y }} ) }
    let right = {
        let _false  = _false.clone();
        r!(move |p: Rp| {
            p.call(_false.clone())
        })
    };

    let pairtest = pair.call(three.clone()).call(five.clone());
    println!("PAIR (3,5) left:{} three:{}",
             to_integer(&left.call(pairtest.clone())),
             to_integer(&right.call(pairtest.clone())));

    // ************************************
    // Arithmetic

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
}
