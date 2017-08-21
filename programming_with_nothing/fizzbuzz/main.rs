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

    // is_zero
    // |n| { n( |x| {FALSE})(TRUE) }
    let is_zero = {
        let _false = _false.clone();
        let _true  = _true.clone();
        let ret_false = r!(move |_x: Rp| _false.clone());
        r!(move |n: Rp| {
            n.call(ret_false.clone()).call(_true.clone())
        })
    };

    println!("IS_ZERO zero:{} three:{}",
             to_boolean(&is_zero.call(zero.clone())),
             to_boolean(&is_zero.call(three.clone())));

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
    // Numeric operation

    // increment
    // |n| { |p| { |x| { p(n(p)(x)) }  } }
    let incr = r!(|n: Rp| {
        r!(move |p: Rp| {
            let n = n.clone();
            let p = p.clone();
            r!(move |x: Rp| p.call(n.call(p.clone()).call(x)))
        })
    });

    // slide
    // |p| { pair(right(p))(incr(right(p))) }
    let slide = {
        let pair = pair.clone();
        let right = right.clone();
        let incr = incr.clone();
        r!(move |p: Rp| {
            pair.call(right.call(p.clone()))
                .call(incr.call(right.call(p.clone())))
        })
    };

    // decrement
    // |n| { left(n(slide)(pair(zero)(zero))) }
    let decr = {
        let left = left.clone();
        let slide = slide.clone();
        let pair = pair.clone();
        let zero = zero.clone();
        r!(move |n: Rp| {
            left.call(n.call(slide.clone())
                       .call(pair.call(zero.clone()).call(zero.clone())))
        })
    };

    // add
    // |m| { |n| { n(incr)(m) } }
    let add = {
        let incr = incr.clone();
        r!(move |m: Rp| {
            let incr = incr.clone();
            let m = m.clone();
            r!(move |n: Rp| {
                n.call(incr.clone()).call(m.clone())
            })
        })
    };

    // substract
    // |m| { |n| { n(decr)(m) } }
    let substract = {
        let decr = decr.clone();
        r!(move |m: Rp| {
            let decr = decr.clone();
            r!(move |n: Rp| {
                n.call(decr.clone()).call(m.clone())
            })
        })
    };

    // multiply
    // |m| { |n| { n(add(m))(zero) } }
    let multiply = {
        let add = add.clone();
        let zero = zero.clone();
        r!(move |m: Rp| {
            let add = add.clone();
            let zero = zero.clone();
            r!(move |n: Rp| {
                n.call(add.clone().call(m.clone()).call(zero.clone()))
            })
        })
    };

    // power
    // |m| { |n| { n(multiply(m))(one) } }
    let power = {
        let multiply = multiply.clone();
        let one = one.clone();
        r!(move |m: Rp| {
            let multiply = multiply.clone();
            let one = one.clone();
            r!(move |n: Rp| {
                n.call(multiply.clone().call(m.clone()).call(one.clone()))
            })
        })
    };

    println!("DECREMENT decr(5):{} decr(0):{} decr(100):{}",
             to_integer(&decr.call(five.clone())),
             to_integer(&decr.call(zero.clone())),
             to_integer(&decr.call(hundred.clone())));

    println!("ARITHMETIC 3+5:{} 5-3:{} 3*5:{} 3**5:{} 2*15: {}",
             to_integer(&add.call(three.clone()).call(five.clone())),
             to_integer(&substract.call(five.clone()).call(three.clone())),
             to_integer(&multiply.call(three.clone()).call(five.clone())),
             to_integer(&power.call(three.clone()).call(five.clone())),
             to_integer(&multiply.call(two.clone()).call(zero.clone())));
}
