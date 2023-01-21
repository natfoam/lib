use core::iter::Rev;

use alloc::vec::Vec;
use uints::Number;

use crate::Node;

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

pub struct LightStack<T: Node> {
    pub stack: Vec<T>,
    pub set: usize,
}

impl<T: Node> Stack for LightStack<T> {
    type Node = T;
    type RevIterator = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: Vec::with_capacity(capacity),
            set: 0,
        }
    }

    fn push(&mut self, value: (Self::Node, u8)) {
        self.stack.push(value.0);
        self.set.set(value.1);
    }

    fn pop(&mut self) -> Option<(Self::Node, u8)> {
        self.stack.pop().map(|v| {
            let level = self.set.trailing_zeros() as u8;
            self.set.unset(level);
            (v, level)
        })
    }

    fn rev_iter(self) -> Self::RevIterator {
        self
    }
}

impl<T: Node> Iterator for LightStack<T> {
    type Item = (T, u8);
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
