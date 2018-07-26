use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt;

use helper::{hashset_eq};

#[derive(Clone)]
pub struct StateSet<T>(pub HashSet<T>);

impl<T: Clone> StateSet<T> {
    pub fn new(t: &HashSet<T>) -> Self{
        StateSet {
            0: t.clone()
        }
    }
}

impl<T: Eq + Clone + Hash> PartialEq for StateSet<T> {
    fn eq(&self, other: &StateSet<T>) -> bool {
        hashset_eq(&self.0, &other.0)
    }
}

impl<T: Eq + Clone + Hash> Eq for StateSet<T> {}

impl<T: Eq + Clone + Hash + Ord> Hash for StateSet<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let mut a: Vec<&T> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for StateSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateSet: {:?}", self.0)
    }
}
