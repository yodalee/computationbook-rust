use std::collections::HashSet;

pub fn toHashSet(arr: &[u32]) -> HashSet<u32> {
    arr.iter().cloned().collect::<HashSet<u32>>()
}
