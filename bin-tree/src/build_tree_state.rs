use uints::Common;

use crate::{node::Node, stack::Stack};

fn state_capacity(v: usize) -> usize {
    ((v + 1) as u64).log2() as usize
}

pub fn new<T: Iterator>(i: &T) -> Stack<(T::Item, usize)> {
    let (min, max) = i.size_hint();
    let capacity = state_capacity(max.unwrap_or(min));
    Stack::with_capacity(capacity)
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
pub fn fold_op<T: Node>(mut stack: Stack<(T, usize)>, mut right: T) -> Stack<(T, usize)> {
    let mut right_level = 0;
    loop {
        match stack.pop() {
            Some(left) => {
                if left.1 == right_level {
                    right = left.0.new_parent2(right);
                    right_level += 1;
                } else {
                    stack.push(left);
                    break;
                }
            }
            _ => break,
        }
    }
    stack.push((right, right_level));
    stack
}

pub fn collect<T: Node>(stack: Stack<(T, usize)>) -> Option<T> {
    stack
        .into_iter()
        .rev()
        .reduce(|(mut right, mut right_level), (left, left_level)| {
            while left_level > right_level {
                right = right.new_parent1();
                right_level += 1;
            }
            (left.new_parent2(right), right_level + 1)
        })
        .map(|(v, _)| v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn state_capacity_test() {
        assert_eq!(state_capacity(0), 0);
        assert_eq!(state_capacity(1), 1);
        assert_eq!(state_capacity(2), 1);
        assert_eq!(state_capacity(3), 2);
        assert_eq!(state_capacity(4), 2);
        assert_eq!(state_capacity(5), 2);
        assert_eq!(state_capacity(6), 2);
        assert_eq!(state_capacity(7), 3);
        assert_eq!(state_capacity(8), 3);
        assert_eq!(state_capacity(0xE), 3);
        assert_eq!(state_capacity(0xF), 4);
        assert_eq!(state_capacity(0x10), 4);
    }
}
