use std::collections::HashSet;
use std::hash::Hash;

pub fn to_hashset<T: Eq + Clone + Hash>(arr: &[T]) -> HashSet<T> {
    arr.iter().cloned().collect::<HashSet<T>>()
}
