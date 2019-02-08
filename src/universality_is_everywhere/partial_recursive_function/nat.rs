#![allow(dead_code)]
use super::prf::{zero, zero_arg, increment, recurse, minimize};
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

fn add_zero_to_x(x: &mut[u32]) -> u32 {
    x.first().unwrap().clone()
}

fn increment_easier_result(
    _x: &mut[u32], _y: &mut u32, result: &mut u32) -> u32 {
    increment(*result)
}

pub fn add(x: u32, y: u32) -> u32 {
    recurse(Rc::new(add_zero_to_x),
            Rc::new(increment_easier_result),
            &mut [x,y])
}

fn multiply_zero_to_x(_x: &mut[u32]) -> u32 {
    zero()
}

fn add_x_to_easier_result(
    x: &mut[u32], _y: &mut u32, result: &mut u32) -> u32 {
    add(*x.first().unwrap(),
        *result)
}

pub fn multiply(x: u32, y: u32) -> u32 {
    recurse(Rc::new(multiply_zero_to_x),
            Rc::new(add_x_to_easier_result),
            &mut [x,y])
}

fn easier_x(
    _x: &mut[u32], y: &mut u32, _result: &mut u32) -> u32 {
    println!("{:?} {:?} {:?}", _x, y, _result);
    *y
}

pub fn decrement(x: u32) -> u32 {
    recurse(Rc::new(zero_arg),
            Rc::new(easier_x),
            &mut[x])
}

fn subtract_zero_from_x(x: &mut[u32]) -> u32 {
    *x.first().unwrap()
}

fn decrement_easier_result(
    _x: &mut[u32], _y: &mut u32, result: &mut u32) -> u32 {
    decrement(*result)
}

pub fn subtract(x: u32, y: u32) -> u32 {
    recurse(Rc::new(subtract_zero_from_x),
            Rc::new(decrement_easier_result),
            &mut[x,y])
}


pub fn divide(x: u32, y: u32) -> u32 {
    minimize(Rc::new(move |n: u32| 
             subtract(
                 increment(x.clone()),
                 multiply(y.clone(), increment(n)))))
}
