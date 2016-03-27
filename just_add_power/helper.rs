use std::collections::HashSet;
use std::hash::{Hash};
use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

pub fn toHashSet<T: Clone+Eq+Hash>(arr: &[T]) -> HashSet<T> {
    arr.iter().cloned().collect::<HashSet<T>>()
}

