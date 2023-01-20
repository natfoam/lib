extern crate alloc;

use alloc::{collections::BTreeSet, vec::Vec};

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

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    fn add(mut self, item: Self::Item) -> Self {
        self.insert(item);
        self
    }
}
