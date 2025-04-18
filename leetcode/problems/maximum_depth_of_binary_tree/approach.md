# Approach for Maximum Depth of Binary Tree

The maximum depth of a binary tree is the number of nodes along the longest path from the root down to the farthest leaf node. This can be found using either recursive or iterative methods.

## 1. Recursive Approach (Depth-First Search - DFS)

This is the most common and often the simplest approach conceptually.

*   **Idea:** The depth of a tree is 1 plus the maximum depth of its left and right subtrees. The depth of an empty tree (null node) is 0.

*   **Process:**
    1.  Define a function `maxDepth(node)`.
    2.  **Base Case:** If `node` is `None` (or null), return 0.
    3.  **Recursive Step:**
        *   Calculate the depth of the left subtree: `left_depth = maxDepth(node.left)`.
        *   Calculate the depth of the right subtree: `right_depth = maxDepth(node.right)`.
        *   Return `1 + max(left_depth, right_depth)`.
    4.  The initial call is `maxDepth(root)`.

*   **Complexity:**
    *   Time: O(N), where N is the number of nodes, as each node is visited once.
    *   Space: O(H), where H is the height of the tree, due to the recursion call stack. O(N) in the worst case (skewed tree), O(log N) for a balanced tree.

## 2. Iterative Approach (Breadth-First Search - BFS)

This approach uses level-order traversal.

*   **Idea:** Traverse the tree level by level. The depth is simply the total number of levels.

*   **Process:**
    1.  If `root` is `None`, return 0.
    2.  Initialize a queue (e.g., `VecDeque` in Rust) and add the `root`.
    3.  Initialize `depth = 0`.
    4.  While the queue is not empty:
        *   Increment `depth` (we are starting a new level).
        *   Get the number of nodes currently in the queue (`level_size = queue.len()`). This represents all nodes at the current depth.
        *   Loop `level_size` times:
            *   Dequeue a node, `current`.
            *   If `current.left` is not `None`, enqueue `current.left`.
            *   If `current.right` is not `None`, enqueue `current.right`.
    5.  Return `depth`.

*   **Complexity:**
    *   Time: O(N), as each node is visited once.
    *   Space: O(W), where W is the maximum width of the tree. O(N) in the worst case (complete binary tree).

## 3. Iterative Approach (Depth-First Search - DFS)

This approach uses a stack to mimic recursion.

*   **Idea:** Use a stack to keep track of nodes to visit along with their current depth from the root.

*   **Process:**
    1.  If `root` is `None`, return 0.
    2.  Initialize a stack and push the tuple `(root, 1)` (node and its depth).
    3.  Initialize `max_depth = 0`.
    4.  While the stack is not empty:
        *   Pop `(current_node, current_depth)` from the stack.
        *   Update `max_depth = max(max_depth, current_depth)`.
        *   If `current_node.left` exists, push `(current_node.left, current_depth + 1)` onto the stack.
        *   If `current_node.right` exists, push `(current_node.right, current_depth + 1)` onto the stack.
    5.  Return `max_depth`.

*   **Complexity:**
    *   Time: O(N).
    *   Space: O(H), the maximum depth of the stack corresponds to the height of the tree.

## Conclusion

All methods correctly find the maximum depth. The recursive DFS is often the most concise. BFS is a good iterative alternative, especially if recursion depth is a concern, while iterative DFS provides another non-recursive option with space complexity related to height.
