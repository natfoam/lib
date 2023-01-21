use alloc::vec::Vec;
use uints::Number;

use crate::{stack::Stack, Node};

pub struct VecStack<T: Node> {
    pub stack: Vec<T>,
    pub set: usize,
}

impl<T: Node> Stack for VecStack<T> {
    type Node = T;
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

    fn pop_if(&mut self, level: u8) -> Option<Self::Node> {
        let left_level = self.set.trailing_zeros() as u8;
        if left_level == level {
            self.set.unset(level);
            self.stack.pop()
        } else {
            None
        }
    }
}

impl<T: Node> Iterator for VecStack<T> {
    type Item = (T, u8);
    fn next(&mut self) -> Option<Self::Item> {
        let VecStack { stack, set } = self;
        stack.pop().map(|v| {
            let level = set.trailing_zeros() as u8;
            set.unset(level);
            (v, level)
        })
    }
}
