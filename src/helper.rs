use std::collections::HashSet;
use std::hash::Hash;

pub fn to_hashset<T: Eq + Clone + Hash>(arr: &[T]) -> HashSet<T> {
    arr.iter().cloned().collect::<HashSet<T>>()
}

pub fn hashset_eq<T: Eq + Clone + Hash>(
    set1: &HashSet<T>, set2: &HashSet<T>) -> bool {
    set1.is_subset(set2) && set1.is_superset(&set2)
}
