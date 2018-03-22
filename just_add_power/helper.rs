use std::collections::HashSet;
use std::hash::{Hash};

pub fn to_hashset<T: Clone+Eq+Hash>(arr: &[T]) -> HashSet<T> {
    arr.iter().cloned().collect::<HashSet<T>>()
}

