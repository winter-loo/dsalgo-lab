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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Early return if any of the inputs are None
        if root.is_none() || p.is_none() || q.is_none() {
            return None;
        }

        // Get the values for comparison
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;

        // Recursive traversal using clones instead of take() to avoid ownership issues
        fn find_lca(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let node_val = node.borrow().val;

                // If both values are less than current node, go left
                if p_val < node_val && q_val < node_val {
                    return find_lca(&node.borrow().left, p_val, q_val);
                }

                // If both values are greater than current node, go right
                if p_val > node_val && q_val > node_val {
                    return find_lca(&node.borrow().right, p_val, q_val);
                }

                // If one value is less and one is greater (or equal to current),
                // then current node is the LCA
                return Some(node.clone());
            }
            None
        }

        // Call the helper function with the values
        find_lca(&root, p_val, q_val)
    }
}
