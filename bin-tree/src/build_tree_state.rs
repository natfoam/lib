use crate::{stack::Stack, Node};

#[repr(transparent)]
pub struct BuildTreeState<S: Stack>(S);

impl<S: Stack> BuildTreeState<S> {
    pub fn new(i: &impl Iterator<Item = S::Node>) -> Self {
        Self(S::with_capacity(i))
    }

    // 00 => 0 []
    // 01 => 1 [0]
    // 02 => 1 [1]
    // 03 => 2 [1,0]
    // 04 => 2 [1,1],[2]
    // 05 => 2 [2,0]
    // 06 => 2 [2,1]
    // 07 => 3 [2,1,0]
    // 08 => 3 [2,2],[3]
    // 09 => 3 [3,0]
    // 0A => 3 [3,1]
    // 0B => 3 [3,1,0]
    // 0C => 3 [3,1,1],[3,2]
    // 0D => 3 [3,2,0]
    // 0E => 3 [3,2,1]
    // 0F => 4 [3,2,1,0]
    // 10 => 4 [3,2,1,1],[3,2,2],[3,3],[4]
    // 11 => 4 [4,0]
    // ...
    // 1E => 4 [4,3,2,1]
    // 1F => 5 [4,3,2,1,0]
    // 20 => 5 [4,3,2,1,1],[4,3,2,2],[4,3,3],[4,4],[5]
    // ...
    // 3E => 5 [5,4,3,2,1]
    // 3F => 6 [5,4,3,2,1,0]
    // 40 => 6 [5,4,3,2,1,1],[5,4,3,2,2],[5,4,3,3],[5,4,4],[5,5],[6]
    pub fn fold_op(mut self, mut right: S::Node) -> Self {
        let mut right_level = 0;
        loop {
            match self.0.pop_if(right_level) {
                Some(left) => {
                    right = left.new_parent(right);
                    right_level += 1;
                }
                _ => break,
            }
        }
        self.0.push((right, right_level));
        self
    }

    pub fn collect(self) -> Option<S::Node> {
        self.0
            .reduce(|(mut right, mut right_level), (left, left_level)| {
                while left_level > right_level {
                    right = right.new_parent_from_single();
                    right_level += 1;
                }
                (left.new_parent(right), right_level + 1)
            })
            .map(|(v, _)| v)
    }
}

#[cfg(test)]
mod tests {
    use crate::VecStack;

    use super::*;

    #[derive(Clone, Default, PartialEq, Eq, Debug)]
    struct Sum(usize);

    impl Node for Sum {
        fn new_parent(self, right: Self) -> Self {
            Sum(self.0 + right.0)
        }

        fn new_parent_from_single(self) -> Self {
            self
        }
    }

    pub struct DebugStack<T: Node> {
        vec: VecStack<T>,
        max_len: usize,
    }

    impl<T: Node> Stack for DebugStack<T> {
        type Node = T;
        fn with_capacity(i: &impl Iterator) -> Self {
            Self {
                vec: VecStack::with_capacity(i),
                max_len: 0,
            }
        }
        fn push(&mut self, value: (T, u8)) {
            self.vec.push(value);
            self.max_len = self.max_len.max(self.vec.stack.len());
        }
        fn pop_if(&mut self, level: u8) -> Option<T> {
            self.vec.pop_if(level)
        }
    }

    impl<T: Node> Iterator for DebugStack<T> {
        type Item = (T, u8);
        fn next(&mut self) -> Option<Self::Item> {
            self.vec.next()
        }
    }

    #[test]
    fn sum() {
        let f = |n| -> Option<usize> {
            let i = (0..n).map(|v| Sum(v));
            let state = BuildTreeState::<DebugStack<_>>::new(&i);
            let capacity = state.0.vec.stack.capacity();
            let new_state = i.fold(state, BuildTreeState::fold_op);
            // `max_len` should be equivalent to `capacity`.
            assert_eq!(new_state.0.max_len, capacity);
            // a `set` should be equivalent to `n`.
            assert_eq!(new_state.0.vec.set, n);
            // the size of the final stack state should be a number of `1` bits in `n`.
            assert_eq!(
                new_state.0.vec.stack.len(),
                n.count_ones() as usize,
                "n: {n}"
            );
            new_state.collect().map(|v| v.0)
        };
        assert_eq!(f(0), None);
        let g = |n| assert_eq!(f(n), Some(n * (n - 1) / 2));
        for i in 1..10_000 {
            g(i);
        }
        g(100_000_000);
    }
}
