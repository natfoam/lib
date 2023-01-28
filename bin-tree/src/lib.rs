//! Building a binary tree from an iterator.
//!
//! # Examples
//!
//! ```
//! use bin_tree::{IteratorEx, Node};
//!
//! #[derive(Clone, Default, PartialEq, Eq, Debug)]
//! struct NodeStr(String);
//!
//! impl Node for NodeStr {
//!     fn new_parent(self, right: Self) -> Self {
//!         Self("[".to_owned() + &self.0 + &right.0 + "]")
//!     }
//!     fn new_parent_from_single(self) -> Self {
//!         self
//!     }
//! }
//!
//! let x = (0..10).map(|v| NodeStr(v.to_string())).build_tree();
//! assert_eq!(x, Some(NodeStr("[[[[01][23]][[45][67]]][89]]".to_string())));
//! ```

#![no_std]
extern crate alloc;

use build_tree_state::BuildTreeState;
pub use node::Node;
use vec_stack::VecStack;

mod build_tree_state;
mod node;
mod stack;
mod vec_stack;

/// The trait extends the functionality of the standard `Iterator` trait by adding
/// the `build_tree` method.
pub trait IteratorEx: Iterator + Sized
where
    Self::Item: Node,
{
    /// Builds a binary tree from an iterator of Nodes
    ///
    /// # Arguments
    ///
    /// * self - the iterator of Nodes to build the tree from
    ///
    /// # Return
    ///
    /// The root node of the built tree, if it was successfully built.
    fn build_tree(self) -> Option<Self::Item> {
        let state = BuildTreeState::<VecStack<_>>::new(&self);
        self.fold(state, BuildTreeState::fold_op).collect()
    }
}

impl<T: Iterator> IteratorEx for T where T::Item: Node {}

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
        let root = (0..11).map(leaf).build_tree().unwrap();
        assert_eq!(55, root.value);
        // [0..7],[8..10]
        let (n0, n1) = root.children.unwrap();
        assert_eq!(28, n0.value);
        assert_eq!(27, n1.value);
        // [0..3],[4..7]
        {
            let (n00, n01) = n0.children.unwrap();
            assert_eq!(6, n00.value);
            assert_eq!(22, n01.value);
            // [0..1],[2..3]
            {
                let (n000, n001) = n00.children.unwrap();
                assert_eq!(1, n000.value);
                assert_eq!(5, n001.value);
                // [0],[1]
                {
                    let (n0000, n0001) = n000.children.unwrap();
                    assert_eq!(0, n0000.value);
                    assert_eq!(1, n0001.value);
                    assert!(n0000.children.is_none());
                    assert!(n0001.children.is_none());
                }
                // [2],[3]
                {
                    let (n0010, n0011) = n001.children.unwrap();
                    assert_eq!(2, n0010.value);
                    assert_eq!(3, n0011.value);
                    assert!(n0010.children.is_none());
                    assert!(n0011.children.is_none());
                }
            }
            // [4..5],[6..7]
            {
                let (n010, n011) = n01.children.unwrap();
                assert_eq!(9, n010.value);
                assert_eq!(13, n011.value);
                // [4],[5]
                {
                    let (n0100, n0101) = n010.children.unwrap();
                    assert_eq!(4, n0100.value);
                    assert_eq!(5, n0101.value);
                    assert!(n0100.children.is_none());
                    assert!(n0101.children.is_none());
                }
                // [6],[7]
                {
                    let (n0110, n0111) = n011.children.unwrap();
                    assert_eq!(6, n0110.value);
                    assert_eq!(7, n0111.value);
                    assert!(n0110.children.is_none());
                    assert!(n0111.children.is_none());
                }
            }
        }
        // [8..9],[10]
        {
            let (n10, n11) = n1.children.unwrap();
            assert_eq!(17, n10.value);
            assert_eq!(10, n11.value);
            assert!(n11.children.is_none());
            // [8],[9]
            {
                let (n100, n101) = n10.children.unwrap();
                assert_eq!(8, n100.value);
                assert_eq!(9, n101.value);
                assert!(n100.children.is_none());
                assert!(n101.children.is_none());
            }
        }
    }
}
