# Approach

To count good nodes in a binary tree, we need to track the maximum value seen so far along each path from the root.

## Strategy
- Use a depth-first search (DFS) traversal to visit each node.
- Keep track of the maximum value seen along the current path.
- If the current node's value is greater than or equal to the maximum value seen so far, it's a good node.

## Steps
1. Define a recursive helper function that takes a node and the maximum value seen so far.
2. If the current node's value is greater than or equal to the maximum, increment the count.
3. Update the maximum value for the recursive calls to the children.
4. Return the total count of good nodes.

// TODO: Add time and space complexity analysis.
