# Hints for Lowest Common Ancestor of a Binary Search Tree

1. Use the BST property to your advantage: all nodes in the left subtree have values less than the current node, and all nodes in the right subtree have values greater.
2. If both p and q are less than the current node, the LCA must be in the left subtree.
3. If both p and q are greater than the current node, the LCA must be in the right subtree.
4. If p and q are on different sides of the current node (or one of them equals the current node), then the current node is the LCA.
5. This problem can be solved iteratively or recursively.
