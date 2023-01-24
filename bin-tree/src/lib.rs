#![no_std]
extern crate alloc;

use build_tree_state::BuildTreeState;
pub use node::Node;
use vec_stack::VecStack;

mod build_tree_state;
mod node;
mod stack;
mod vec_stack;

/// The trait should extend the functionality of a sequence by adding the `build_tree` method.
pub trait Sequence {
    /// A node of the sequence.
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
impl<T: Iterator> Sequence for T
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
    use alloc::boxed::Box;

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

    #[test]
    fn sum() {
        let x = (0..10).map(|v| Sum(v)).build_tree();
        assert_eq!(x, Some(Sum(45)));
    }

    struct NodeContent {
        value: usize,
        children: Option<(NodeRef, NodeRef)>,
    }

    type NodeRef = Box<NodeContent>;

    fn leaf(value: usize) -> NodeRef {
        Box::new(NodeContent {
            value,
            children: None,
        })
    }

    impl Node for NodeRef {
        fn new_parent(self, right: Self) -> Self {
            let value = self.value + right.value;
            Box::new(NodeContent {
                value,
                children: Some((self, right)),
            })
        }

        fn new_parent_from_single(self) -> Self {
            self
        }
    }

    #[test]
    fn node() {
        let root = (0..10).map(leaf).build_tree().unwrap();
        assert_eq!(45, root.value);
        //
        let (n0, n1) = root.children.unwrap();
        assert_eq!(28, n0.value);
        assert_eq!(17, n1.value);
        //
        let (n00, n01) = n0.children.unwrap();
        assert_eq!(6, n00.value);
        assert_eq!(22, n01.value);
        //
        let (n000, n001) = n00.children.unwrap();
        assert_eq!(1, n000.value);
        assert_eq!(5, n001.value);
        //
        let (n0000, n0001) = n000.children.unwrap();
        assert_eq!(0, n0000.value);
        assert_eq!(1, n0001.value);
        assert!(n0000.children.is_none());
        assert!(n0001.children.is_none());
    }
}
