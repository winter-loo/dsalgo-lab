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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        height(root).1
    }
}

fn height(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    if let Some(root) = root {
        let mut root = root.borrow_mut();
        let (h1, b1) = height(root.left.take());
        if !b1 {
            return (h1, false);
        }
        let (h2, b2) = height(root.right.take());
        if !b2 {
            return (h2, false);
        }
        (1 + h1.max(h2), (h1 - h2).abs() <= 1)
    } else {
        (0, true)
    }
}
