use core::iter::Rev;

use alloc::vec::Vec;

pub trait Stack
{
    type Item;
    type RevIterator: Iterator<Item = Self::Item>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    fn rev_iter(self) -> Self::RevIterator;
}

impl<T> Stack for Vec<T> {
    type Item = T;
    type RevIterator = Rev<<Self as IntoIterator>::IntoIter>;
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn push(&mut self, value: Self::Item) {
        self.push(value)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn rev_iter(self) -> Self::RevIterator {
        self.into_iter().rev()
    }
}
