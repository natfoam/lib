use alloc::vec::Vec;
use uints::{Number, log2, is_set, unset};

use crate::{stack::Stack, Node};

pub struct VecStack<T: Node> {
    pub stack: Vec<T>,
    pub set: usize,
}

impl<T: Node> Stack for VecStack<T> {
    type Node = T;
    fn with_capacity(i: &impl Iterator) -> Self {
        let (min, max) = i.size_hint();
        let size = max.unwrap_or(min);
        let capacity = log2(size + 1) as usize;
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
        let Self { stack, set } = self;
        let s = *set;
        if is_set(s, level) {
            *set = unset(s, level);
            stack.pop()
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
            let s = *set;
            let level = s.trailing_zeros() as u8;
            *set = unset(s, level);
            (v, level)
        })
    }
}
