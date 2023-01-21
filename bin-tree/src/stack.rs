use alloc::vec::Vec;
use uints::Number;

use crate::Node;

pub trait Stack
{
    type Node;
    type RevIterator: Iterator<Item = (Self::Node, u8)>;
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: (Self::Node, u8));
    fn pop(&mut self, level: u8) -> Option<Self::Node>;
    fn rev_iter(self) -> Self::RevIterator;
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

    fn pop(&mut self, level: u8) -> Option<Self::Node> {
        let left_level = self.set.trailing_zeros() as u8;
        if left_level == level {
            self.set.unset(level);
            self.stack.pop()
        } else {
            None
        }
    }

    fn rev_iter(self) -> Self::RevIterator {
        self
    }
}

impl<T: Node> Iterator for LightStack<T> {
    type Item = (T, u8);
    fn next(&mut self) -> Option<Self::Item> {
        let LightStack { stack, set } = self;
        stack.pop().map(|v| {
            let level = set.trailing_zeros() as u8;
            set.unset(level);
            (v, level)
        })
    }
}
