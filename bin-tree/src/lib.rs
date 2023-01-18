pub trait Node {
    fn new2(&self, right: &Self) -> Self;
    fn new1(&self) -> Self;
}

#[repr(transparent)]
pub struct State<T: Node>(Vec<(T, u8)>);

fn state_capacity(v: usize) -> usize {
    if v < 2 {
        0
    } else {
        (usize::BITS - (v + 1).leading_zeros() - 1) as usize
    }
}

impl<T: Node> State<T> {
    pub fn new(i: &impl Iterator<Item = T>) -> Self {
        State(Vec::with_capacity(state_capacity(match i.size_hint() {
            (_, Some(upper)) => upper,
            (lower, _) => lower,
        })))
    }
    // 00 => 0 []
    // 01 => 0 []
    // 02 => 1 [0],[1]
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
    pub fn fold_op(mut self, mut right: T) -> Self {
        let mut right_level = 0;
        loop {
            match self.0.last() {
                Some((left, left_level)) if *left_level == right_level => {
                    right = left.new2(&right);
                    right_level += 1;
                    self.0.pop();
                }
                _ => break,
            }
        }
        self.0.push((right, right_level));
        self
    }
    pub fn collect(self) -> Option<T> {
        self.0
            .into_iter()
            .rev()
            .reduce(|(mut right, mut right_level), (left, left_level)| {
                while left_level > right_level {
                    right = right.new1();
                    right_level += 1;
                }
                (left.new2(&right), right_level + 1)
            })
            .map(|(v, _)| v)
    }
}

pub trait BinTree {
    type Result: Node;
    fn bin_tree(self) -> Option<Self::Result>;
}

impl<T: Iterator> BinTree for T
where
    T::Item: Node,
{
    type Result = T::Item;
    fn bin_tree(self) -> Option<Self::Result> {
        let state = State::new(&self);
        self.fold(state, State::fold_op).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, PartialEq, Eq, Debug)]
    struct Sum(usize);

    impl Node for Sum {
        fn new2(&self, right: &Self) -> Self {
            Sum(self.0 + right.0)
        }

        fn new1(&self) -> Self {
            self.clone()
        }
    }

    #[test]
    fn sum() {
        let x = (0..10).map(|v| Sum(v)).bin_tree();
        assert_eq!(x, Some(Sum(45)));
    }

    #[test]
    fn state_capacity_test() {
        assert_eq!(state_capacity(0), 0);
        assert_eq!(state_capacity(1), 0);
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
