# Binary Tree Builder

The library provides an `IteratorEx` trait that has a `build_tree` method for any iterator of nodes that implement the `Node` trait.
The `build_tree` function returns the root node of the built tree if it was successfully built, otherwise it returns `None` if the provided
iterator is empty.

## Node Trait

The `Node` trait represents a node in a binary tree. It provides two methods for creating a new parent node from child nodes:

- `new_parent2(&self, right: &Self) -> Self`: creates a new parent node from two child nodes,
- `new_parent1(&self) -> Self`: creates a new parent node from one child node.

The implementation of these methods is specific to the type of node being used, and should be provided by the user.