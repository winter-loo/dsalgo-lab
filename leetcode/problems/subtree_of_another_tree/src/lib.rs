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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        subroot: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // Helper function to check if two trees are identical
        fn is_same_tree(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p_borrow = p.borrow();
                    let q_borrow = q.borrow();
                    p_borrow.val == q_borrow.val
                        && is_same_tree(p_borrow.left.clone(), q_borrow.left.clone())
                        && is_same_tree(p_borrow.right.clone(), q_borrow.right.clone())
                }
                _ => false,
            }
        }

        // If subroot is empty, it's always a subtree
        if subroot.is_none() {
            return true;
        }

        // If root is empty but subroot isn't, it can't be a subtree
        if root.is_none() {
            return false;
        }

        // Check if the current root tree is identical to subroot
        // or if subroot is a subtree of root's left or right subtree
        is_same_tree(root.clone(), subroot.clone())
            || Self::is_subtree(
                root.as_ref().unwrap().borrow().left.clone(),
                subroot.clone(),
            )
            || Self::is_subtree(
                root.as_ref().unwrap().borrow().right.clone(),
                subroot.clone(),
            )
    }
}
