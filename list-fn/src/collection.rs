use std::collections::{BTreeSet, HashSet};

use std::hash::Hash;

pub trait Collection {
    type Item;
    fn add(self, item: Self::Item) -> Self; 
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn add(mut self, item: Self::Item) -> Self {
        self.push(item);
        self
    }
}

impl<T: Hash + Eq> Collection for HashSet<T> {
    type Item = T;

    fn add(mut self, item: Self::Item) -> Self {
        self.insert(item);
        self
    }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    fn add(mut self, item: Self::Item) -> Self {
        self.insert(item);
        self
    }
}