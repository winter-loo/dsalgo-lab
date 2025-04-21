# Approach

To construct a binary tree from preorder and inorder traversals, we can use the properties of these traversals to recursively build the tree.

## Strategy
- The first element in the preorder traversal is always the root of the tree.
- In the inorder traversal, elements to the left of the root belong to the left subtree, and elements to the right belong to the right subtree.
- We can recursively apply this logic to build the entire tree.

## Steps
1. Use the first element of the preorder array as the root.
2. Find the position of this root element in the inorder array.
3. Elements to the left of this position in the inorder array form the left subtree.
4. Elements to the right of this position in the inorder array form the right subtree.
5. Recursively build the left and right subtrees using the corresponding portions of the preorder and inorder arrays.

## Optimization
- Use a hashmap to store the indices of elements in the inorder array for O(1) lookup.
- Track the ranges of the preorder and inorder arrays to avoid creating new subarrays.

// TODO: Add time and space complexity analysis.
