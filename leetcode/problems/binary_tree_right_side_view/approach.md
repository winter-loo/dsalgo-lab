# Approach

To find the right side view of a binary tree, we need to identify the rightmost node at each level of the tree.

## Strategy
- Use a level-order traversal (BFS) to visit nodes level by level.
- For each level, add the last node's value to the result.
- Alternatively, use a modified DFS where we prioritize the right subtree before the left.

## Steps
1. Use a queue to perform a level-order traversal.
2. At each level, keep track of the last node visited.
3. After processing each level, add the value of the last node to the result.

// TODO: Add time and space complexity analysis.
