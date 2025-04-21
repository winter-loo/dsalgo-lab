# Approach

To find the kth smallest element in a binary search tree (BST), we can leverage the property that an in-order traversal of a BST visits the nodes in ascending order.

## Strategy
- Perform an in-order traversal of the BST.
- Keep track of the number of nodes visited.
- When we've visited k nodes, we've found our answer.

## Steps
1. Initialize a counter to track the number of nodes visited.
2. Perform an in-order traversal (left, root, right).
3. For each node:
   - Recursively traverse the left subtree.
   - Increment the counter.
   - If the counter equals k, we've found our answer.
   - Otherwise, recursively traverse the right subtree.

## Follow-up Optimization
For the follow-up question about frequent modifications and queries:
- Consider augmenting each node with a field that stores the size of its left subtree.
- This allows us to determine in O(log n) time which subtree contains the kth smallest element.

// TODO: Add time and space complexity analysis.
