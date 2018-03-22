use std::collections::HashSet;

pub fn to_hashset(arr: &[u32]) -> HashSet<u32> {
    arr.iter().cloned().collect::<HashSet<u32>>()
}
