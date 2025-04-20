# Approach

To determine if a tree is a subtree of another tree, we need to check if there's a node in the main tree such that the subtree rooted at that node is identical to the second tree.

## Strategy
- Traverse the main tree and at each node, check if the subtree rooted at that node is identical to the second tree.
- Use a helper function to check if two trees are identical (similar to the "Same Tree" problem).

## Steps
1. If the second tree (subRoot) is null, return true (an empty tree is a subtree of any tree).
2. If the main tree (root) is null, return false (a non-empty tree cannot be a subtree of an empty tree).
3. Check if the current node in the main tree is the root of a subtree identical to the second tree.
4. If not, recursively check if the second tree is a subtree of the left or right subtree of the main tree.

// TODO: Add time and space complexity analysis.
