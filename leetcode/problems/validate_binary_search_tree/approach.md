# Approach

To validate if a binary tree is a valid binary search tree (BST), we need to ensure that all nodes in the left subtree are less than the current node, and all nodes in the right subtree are greater than the current node.

## Strategy
- Use a recursive approach to validate each subtree.
- Keep track of the valid range for each node's value.
- For the left subtree, the upper bound is the parent's value.
- For the right subtree, the lower bound is the parent's value.

## Steps
1. Define a helper function that takes a node and the valid range (min and max values).
2. If the node's value is outside the valid range, return false.
3. Recursively validate the left subtree with an updated upper bound.
4. Recursively validate the right subtree with an updated lower bound.
5. Return true if both subtrees are valid.

// TODO: Add time and space complexity analysis.
