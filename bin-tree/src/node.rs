/// A trait representing a node in a binary tree.
pub trait Node {
    /// Creates a new parent node from two child nodes
    ///
    /// # Arguments
    ///
    /// * `right` - the right child node
    ///
    /// # Return
    ///
    /// The new parent node with the left node as `self` and the right node as `right`
    fn new_parent(self, right: Self) -> Self;
    /// Creates a new parent node from a single child node
    ///
    /// # Return
    ///
    /// The new parent node with the child node as `self`
    fn new_parent_from_single(self) -> Self;
}
