# Approach

To find the diameter of a binary tree, we need to compute the length of the longest path between any two nodes in the tree. This path may or may not pass through the root.

## Strategy
- Use depth-first search (DFS) to compute the depth of each subtree.
- At each node, the diameter passing through it is the sum of the depths of its left and right subtrees.
- Track the maximum diameter found during traversal.

## Steps
1. Define a helper function to compute the depth of a node.
2. At each node, update the maximum diameter.
3. Return the maximum diameter after traversing the entire tree.

// TODO: Add time and space complexity analysis.
