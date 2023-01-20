use core::iter::Rev;

use alloc::vec::Vec;

pub trait Stack
{
    type Node;
    type RevIterator: Iterator<Item = (Self::Node, u8)>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: (Self::Node, u8));
    fn pop(&mut self) -> Option<(Self::Node, u8)>;
    fn rev_iter(self) -> Self::RevIterator;
}

impl<T> Stack for Vec<(T, u8)> {
    type Node = T;
    type RevIterator = Rev<<Self as IntoIterator>::IntoIter>;
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn push(&mut self, value: (Self::Node, u8)) {
        self.push(value)
    }

    fn pop(&mut self) -> Option<(Self::Node, u8)> {
        self.pop()
    }

    fn rev_iter(self) -> Self::RevIterator {
        self.into_iter().rev()
    }
}
