#![no_std]
extern crate alloc;

use build_tree_state::BuildTreeState;
pub use node::Node;
use vec_stack::VecStack;

mod build_tree_state;
mod node;
mod stack;
mod vec_stack;

pub trait IteratorEx {
    type Node: Node;
    /// Builds a binary tree from an iterator of Nodes
    ///
    /// # Arguments
    ///
    /// * self - the iterator of Nodes to build the tree from
    ///
    /// # Return
    ///
    /// The root node of the built tree, if it was successfully built.
    fn build_tree(self) -> Option<Self::Node>;
}

/// The trait extends the functionality of the standard `Iterator` trait by adding
/// the `build_tree` method.
impl<T: Iterator> IteratorEx for T
where
    T::Item: Node,
{
    type Node = T::Item;
    fn build_tree(self) -> Option<Self::Node> {
        let state = BuildTreeState::<VecStack<_>>::new(&self);
        self.fold(state, BuildTreeState::fold_op).collect()
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
