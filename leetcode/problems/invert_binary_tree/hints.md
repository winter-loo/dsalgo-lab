# Hints for Invert Binary Tree

## Key Questions

*   What does it mean to "invert" a binary tree?
*   If you are at a specific node, what needs to happen to its children to contribute to the overall inversion?
*   What is a suitable traversal method (Depth-First Search - DFS, Breadth-First Search - BFS) for visiting every node?
*   What is the base case for a recursive solution?
*   How does swapping the left and right children of a node affect the subtrees rooted at those children?

## Additional Hints

*   **Recursive Thinking:** Think about the simplest case: a leaf node. Inverting a leaf node doesn't change anything. What about a node with one child? What about a node with two children? If you invert the children of a node, what else needs to be done for the subtrees rooted at those children?

*   **Base Case:** If the current node is `None` (or null), there's nothing to invert, so you can just return `None`.

*   **Recursive Step:** For a non-null node:
    1.  Recursively invert the left subtree.
    2.  Recursively invert the right subtree.
    3.  Swap the (now inverted) left and right children of the current node.
    4.  Return the current node.

*   **Order of Operations:** Does it matter if you swap the children *before* or *after* recursively inverting the subtrees? Try thinking through a small example. (Hint: It usually doesn't matter for correctness, but swapping first can be slightly cleaner conceptually).

*   **Iterative Approach (BFS):** You can also solve this iteratively using a queue (like in Breadth-First Search).
    1.  Initialize a queue and add the `root` node (if it's not null).
    2.  While the queue is not empty:
        *   Dequeue a node (`current`).
        *   Swap its left and right children.
        *   Enqueue the left child (if it exists).
        *   Enqueue the right child (if it exists).
    3.  Return the original `root`.

*   **Iterative Approach (DFS):** An iterative DFS approach using a stack is also possible, following a similar pattern to recursive DFS but managing the stack explicitly.
