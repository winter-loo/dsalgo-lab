# Hints for Validate Binary Search Tree

1. Remember that a BST requires all nodes in the left subtree to be less than the current node's value, and all nodes in the right subtree to be greater.
2. It's not enough to just check the immediate children. The entire subtree must satisfy the BST property.
3. Consider using a recursive approach with valid ranges for each node.
4. Be careful with integer overflow/underflow when dealing with the constraints.
5. An in-order traversal of a BST should yield values in ascending order.
