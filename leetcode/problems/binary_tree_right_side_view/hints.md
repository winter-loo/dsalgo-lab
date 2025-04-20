# Hints for Binary Tree Right Side View

1. The right side view contains the last node we see at each level when viewing from the right.
2. Consider using a level-order traversal (BFS) to visit nodes level by level.
3. Alternatively, a modified DFS visiting right children before left children can work if you track the depth.
4. Remember to handle edge cases like an empty tree.
