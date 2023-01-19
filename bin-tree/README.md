# Binary Tree Builder

This library provides a `build_tree` function to building a binary tree from an iterator of nodes.

## Node trait

The `Node` trait represents a node in a binary tree. It provides two methods for creating a new parent node from child nodes:

- `new_parent2(&self, right: &Self) -> Self`: creates a new parent node from two child nodes,
- `new_parent1(&self) -> Self`: creates a new parent node from a child node.

The implementation of these methods is specific to the type of node being used, and should be provided by the user.