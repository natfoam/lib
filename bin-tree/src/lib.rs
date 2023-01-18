pub trait Node {
    fn new2(&self, right: &Self) -> Self;
    fn new1(&self) -> Self;
}

#[repr(transparent)]
pub struct State<T: Node>(Vec<(T, u8)>);

impl<T: Node> Default for State<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Node> State<T> {
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
where T::Item: Node {
    type Result = T::Item;
    fn bin_tree(self) -> Option<Self::Result> {
        self.fold(State::default(), State::fold_op).collect()
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
        let x = (0..10)
            .map(|v| Sum(v))
            .bin_tree();
        assert_eq!(x, Some(Sum(45)));
    }
}
