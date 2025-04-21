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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        dfs(root, &mut k).unwrap_or(-1)
    }
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
    if root.is_none() {
        return None;
    }
    let root = root.unwrap();
    let root = root.borrow();
    if let Some(n) = dfs(root.left.clone(), k) {
        return Some(n);
    }
    *k -= 1;
    if *k == 0 {
        return Some(root.val);
    }
    dfs(root.right.clone(), k)
}
