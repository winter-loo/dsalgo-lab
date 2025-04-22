# Approach

To find the maximum path sum in a binary tree, we need to consider paths that can start and end at any node, not just the root.

## Strategy
- Use a recursive approach to compute the maximum path sum for each subtree.
- For each node, we need to calculate two values:
  1. The maximum path sum that includes the current node and at most one of its subtrees (this can be extended by parent nodes)
  2. The maximum path sum that includes the current node and possibly both of its subtrees (this forms a complete path)
- Keep track of the global maximum path sum across all possible paths.

## Steps
1. Define a recursive function that returns the maximum path sum starting from the current node and going down (can only include at most one subtree).
2. For each node, calculate:
   - The maximum path sum from the left subtree (if positive)
   - The maximum path sum from the right subtree (if positive)
   - The maximum path sum that includes the current node and possibly both subtrees
3. Update the global maximum with the maximum path sum at each node.
4. Return the maximum path sum that can be extended (current node + at most one subtree).

## Key Insight
- Negative path sums from subtrees should be ignored as they would only decrease the overall path sum.
- The global maximum needs to be tracked separately from the return value of the recursive function.

// TODO: Add time and space complexity analysis.
