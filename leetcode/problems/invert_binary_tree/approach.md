# Approach for Invert Binary Tree

Inverting a binary tree means that for every node, its left child becomes its right child, and its right child becomes its left child. This operation needs to be performed recursively or iteratively throughout the tree.

## 1. Recursive Approach (Depth-First Search - DFS)

This is often the most straightforward and intuitive way to solve the problem.

*   **Idea:** Recursively traverse the tree. At each node, first recursively invert its left and right subtrees, and then swap the left and right child pointers of the current node.

*   **Process:**
    1.  Define a function `invertTree(node)`.
    2.  **Base Case:** If `node` is `None` (or null), return `None`.
    3.  **Recursive Step:**
        *   Recursively call `invertTree` on the `node.left`.
        *   Recursively call `invertTree` on the `node.right`.
        *   Swap the children: Store `node.left` in a temporary variable, set `node.left = node.right`, and set `node.right` to the temporary variable.
        *   Return the `node`.
    4.  The initial call would be `invertTree(root)`.

*   **Alternative Order:** You can also swap the children *before* the recursive calls. The result is the same:
    1.  Base Case: If `node` is `None`, return `None`.
    2.  Swap the children of the current `node`.
    3.  Recursively call `invertTree` on the *new* `node.left` (which was the original right child).
    4.  Recursively call `invertTree` on the *new* `node.right` (which was the original left child).
    5.  Return the `node`.

*   **Complexity:**
    *   Time: O(N), where N is the number of nodes in the tree, as each node is visited exactly once.
    *   Space: O(H), where H is the height of the tree. This is due to the recursion call stack. In the worst case (a skewed tree), H can be N, making the space complexity O(N). For a balanced tree, it's O(log N).

## 2. Iterative Approach (Breadth-First Search - BFS)

This approach avoids recursion and uses a queue.

*   **Idea:** Visit nodes level by level. For each visited node, swap its children and add the children to the queue to be processed later.

*   **Process:**
    1.  If `root` is `None`, return `None`.
    2.  Create a queue (e.g., `VecDeque` in Rust) and enqueue the `root`.
    3.  While the queue is not empty:
        *   Dequeue a node, let's call it `current`.
        *   Swap `current.left` and `current.right`.
        *   If `current.left` is not `None`, enqueue `current.left`.
        *   If `current.right` is not `None`, enqueue `current.right`.
    4.  Return the original `root` (which is now the root of the inverted tree).

*   **Complexity:**
    *   Time: O(N), as each node is visited and processed once.
    *   Space: O(W), where W is the maximum width of the tree. In the worst case (a complete binary tree), the width can be roughly N/2, making the space complexity O(N).

## 3. Iterative Approach (Depth-First Search - DFS)

This approach also avoids recursion but uses a stack.

*   **Idea:** Mimic the recursive DFS using an explicit stack.

*   **Process:**
    1.  If `root` is `None`, return `None`.
    2.  Create a stack and push the `root` onto it.
    3.  While the stack is not empty:
        *   Pop a node, `current`.
        *   Swap `current.left` and `current.right`.
        *   If `current.right` is not `None`, push `current.right` onto the stack.
        *   If `current.left` is not `None`, push `current.left` onto the stack. (Pushing right then left ensures a pre-order like traversal processing).
    4.  Return the original `root`.

*   **Complexity:**
    *   Time: O(N).
    *   Space: O(H), similar to recursive DFS, as the stack depth corresponds to the tree height.

## Conclusion

All three approaches are valid and have the same time complexity. The recursive DFS is often the most concise and easiest to understand. The iterative BFS might be preferred in some contexts to avoid potential stack overflow on very deep trees, and its space complexity depends on the tree's width rather than height. The iterative DFS provides an alternative non-recursive method with space complexity related to height.
