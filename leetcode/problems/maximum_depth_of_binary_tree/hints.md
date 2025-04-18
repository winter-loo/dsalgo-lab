# Hints for Maximum Depth of Binary Tree

## Key Questions

*   What is the definition of a tree's depth?
*   What is the depth of an empty tree (a null root)?
*   What is the depth of a tree with only a root node?
*   How does the depth of a node relate to the depths of its left and right subtrees?
*   Which tree traversal method seems most natural for calculating depth?

## Additional Hints

*   **Recursive Thinking (DFS):** This is often the most intuitive approach.
    *   **Base Case:** If the current node is `None` (or null), its depth is 0.
    *   **Recursive Step:** For a non-null node, its maximum depth is 1 (for the node itself) plus the maximum of the depths of its left and right subtrees.
    *   Formula: `depth(node) = 1 + max(depth(node.left), depth(node.right))`

*   **Iterative Thinking (BFS):** You can use level-order traversal.
    *   Keep track of the current level number.
    *   Initialize a queue with the root and the level number (e.g., `(root, 1)`).
    *   Process nodes level by level. The depth of the tree is simply the level number of the last level processed.
    *   You need a way to know when one level ends and the next begins (e.g., process all nodes currently in the queue in an inner loop for each level, or store the level with each node in the queue).

*   **Iterative Thinking (DFS):** You can use a stack.
    *   Instead of just pushing nodes, push pairs `(node, current_depth)` onto the stack.
    *   Initialize the stack with `(root, 1)`.
    *   Keep track of the maximum depth seen so far.
    *   When popping `(node, depth)`, update the maximum depth if `depth` is greater.
    *   Push `(node.left, depth + 1)` and `(node.right, depth + 1)` if they exist.

*   **Simplification:** The core idea is that the longest path must go through either the left child or the right child. Find the length of the longest path in the left subtree and the longest path in the right subtree, take the maximum of those two, and add 1 (for the edge connecting the root to that subtree).
