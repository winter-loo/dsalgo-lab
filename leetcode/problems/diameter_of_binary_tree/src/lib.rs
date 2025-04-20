// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left = depth(&n.left, max_diameter);
                let right = depth(&n.right, max_diameter);
                *max_diameter = (*max_diameter).max(left + right);
                1 + left.max(right)
            } else {
                0
            }
        }
        let mut max_diameter = 0;
        depth(&root, &mut max_diameter);
        max_diameter
    }
}
