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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut nodes = vec![];
        dfs(&root, &mut nodes)
    }
}

// in-order traversal
fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) -> bool {
    if root.is_none() {
        return true;
    }
    let root = root.as_ref().unwrap();
    let root = root.borrow();
    if !dfs(&root.left, nodes) {
        return false;
    }
    if let Some(last) = nodes.last() {
        if root.val <= *last {
            return false;
        }
        nodes.push(root.val);
    } else {
        nodes.push(root.val)
    }

    // we could shrink the nodes here if we have too many nodes

    if !dfs(&root.right, nodes) {
        return false;
    }
    true
}
