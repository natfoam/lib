use build_tree_state::{collect, fold_op, new};
pub use node::Node;

mod build_tree_state;
mod node;
mod stack;

pub trait IteratorEx {
    type Item: Node;
    /// Builds a binary tree from an iterator of Nodes
    ///
    /// # Arguments
    ///
    /// * self - the iterator of Nodes to build the tree from
    ///
    /// # Return
    ///
    /// The root node of the built tree, if it was successfully built.
    fn build_tree(self) -> Option<Self::Item>;
}

/// The trait extends the functionality of the standard `Iterator` trait by adding
/// the `build_tree` method.
impl<T: Iterator> IteratorEx for T
where
    T::Item: Node,
{
    type Item = T::Item;
    fn build_tree(self) -> Option<Self::Item> {
        let state = new(&self);
        collect(self.fold(state, fold_op))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, PartialEq, Eq, Debug)]
    struct Sum(usize);

    impl Node for Sum {
        fn new_parent2(self, right: Self) -> Self {
            Sum(self.0 + right.0)
        }

        fn new_parent1(self) -> Self {
            self
        }
    }

    #[test]
    fn sum() {
        let x = (0..10).map(|v| Sum(v)).build_tree();
        assert_eq!(x, Some(Sum(45)));
    }
}
