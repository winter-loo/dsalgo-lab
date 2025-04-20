# Approach

To determine if a binary tree is height-balanced, check that for every node, the heights of the left and right subtrees differ by no more than one, and both subtrees are balanced.

## Strategy
- Use a recursive helper function to compute the height of each subtree.
- If any subtree is not balanced, propagate that information up.

## Steps
1. Define a helper function that returns the height if the subtree is balanced, or a sentinel value (e.g., -1) if not.
2. At each node, check the heights of left and right subtrees.
3. If the difference is more than 1, return not balanced.

// TODO: Add time and space complexity analysis.
