// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }

        // Create a hashmap to store the index of each value in the inorder array
        // This allows us to find the position of the root value in O(1) time
        let mut inorder_map = std::collections::HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }

        // Call the recursive helper function
        Self::build_tree_helper(
            &preorder,
            0,
            preorder.len() - 1,
            &inorder,
            0,
            inorder.len() - 1,
            &inorder_map,
        )
    }

    fn build_tree_helper(
        preorder: &[i32],
        pre_start: usize,
        pre_end: usize,
        inorder: &[i32],
        in_start: usize,
        in_end: usize,
        inorder_map: &std::collections::HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Base case: if we've processed all elements
        if pre_start > pre_end || in_start > in_end {
            return None;
        }

        // The first element in preorder is the root of the current subtree
        let root_val = preorder[pre_start];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        // Find the position of the root value in the inorder array
        let in_root_idx = *inorder_map.get(&root_val).unwrap();

        // Calculate the size of the left subtree
        let left_subtree_size = in_root_idx as i32 - in_start as i32;

        // Recursively build the left subtree
        if left_subtree_size > 0 {
            root.borrow_mut().left = Self::build_tree_helper(
                preorder,
                pre_start + 1,
                pre_start + left_subtree_size as usize,
                inorder,
                in_start,
                in_root_idx - 1,
                inorder_map,
            );
        }

        // Recursively build the right subtree
        root.borrow_mut().right = Self::build_tree_helper(
            preorder,
            pre_start + left_subtree_size as usize + 1,
            pre_end,
            inorder,
            in_root_idx + 1,
            in_end,
            inorder_map,
        );

        Some(root)
    }
}
