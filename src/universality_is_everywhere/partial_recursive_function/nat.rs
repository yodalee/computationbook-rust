#![allow(dead_code)]
use super::prf::{zero, increment, recurse, minimize, F, G};
use std::rc::{Rc};

pub fn two() -> u32 {
   increment(increment(zero()))
}

pub fn three() -> u32 {
   increment(increment(increment(zero())))

}

pub fn add_three(x: u32) -> u32 {
    increment(increment(increment(x)))
}

pub fn add(x: u32, y: u32) -> u32 {
    let add_zero_to_x : F = Rc::new(
        |x: &[u32]| x.first().unwrap().clone() );
    let increment_easier_result : G = Rc::new(
        |_x: &[u32], _y: &u32, result: &u32| increment(*result) );
    recurse(add_zero_to_x,
            increment_easier_result,
            &mut [x,y])
}

fn add_x_to_easier_result(
    x: &[u32], _y: &u32, result: &u32) -> u32 {
    add(*x.first().unwrap(),
        *result)
}

pub fn multiply(x: u32, y: u32) -> u32 {
    let multiply_zero_to_x : F = Rc::new(
        |_x: &[u32]| zero());
    let add_x_to_easier_result : G = Rc::new(
        |x: &[u32], _y: &u32, result: &u32|
        add(*x.first().unwrap(), *result));
    recurse(multiply_zero_to_x,
            add_x_to_easier_result,
            &mut [x,y])
}

pub fn decrement(x: u32) -> u32 {
    let zero_arg : F = Rc::new(
        |_x: &[u32]| zero());
    let easier_x : G = Rc::new(
        |_x: &[u32], y: &u32, _result: &u32| *y);
    recurse(zero_arg, easier_x, &mut[x])
}

pub fn subtract(x: u32, y: u32) -> u32 {
    let subtract_zero_from_x : F = Rc::new(
        |x: &[u32]| *x.first().unwrap());
    let decrement_easier_result : G = Rc::new(
        |_x: &[u32], _y: &u32, result: &u32|
        decrement(*result));
    recurse(subtract_zero_from_x,
            decrement_easier_result,
            &mut[x,y])
}


pub fn divide(x: u32, y: u32) -> u32 {
    minimize(Rc::new(move |n: &u32|
             subtract(
                 increment(x.clone()),
                 multiply(y.clone(), increment(*n)))))
}
