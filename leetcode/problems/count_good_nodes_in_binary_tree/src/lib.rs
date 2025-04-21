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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut path = vec![];
        let mut count = 0;
        traverse(root, &mut path, &mut count);
        count as i32
    }
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, count: &mut usize) {
    if root.is_none() {
        return;
    }
    let root = root.unwrap();
    let val = root.borrow().val;
    if let Some(last) = path.last() {
        if val >= *last {
            path.push(val);
            *count += 1;
        }
    } else {
        path.push(val);
        *count += 1;
    }

    traverse(root.borrow().left.clone(), path, count);
    traverse(root.borrow().right.clone(), path, count);

    if path.last().is_some_and(|last| *last == val) {
        path.pop();
    }
}
