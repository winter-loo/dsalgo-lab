# Approach

To find the lowest common ancestor (LCA) in a binary search tree (BST), we can leverage the BST property: for any node, all values in its left subtree are less than the node's value, and all values in its right subtree are greater.

## Strategy
- Start from the root and traverse the tree.
- If both p and q are less than the current node's value, the LCA must be in the left subtree.
- If both p and q are greater than the current node's value, the LCA must be in the right subtree.
- If one is less and one is greater (or equal), then the current node is the LCA.

## Steps
1. Compare the values of p and q with the current node's value.
2. If both are less, recursively search in the left subtree.
3. If both are greater, recursively search in the right subtree.
4. Otherwise, the current node is the LCA.

// TODO: Add time and space complexity analysis.
