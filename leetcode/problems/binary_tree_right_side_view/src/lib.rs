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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut level_nodes = vec![];
        myimpl(root, 0, &mut level_nodes);
        level_nodes.iter().flat_map(|v| v.iter()).copied().collect()
    }
}

fn myimpl(root: Option<Rc<RefCell<TreeNode>>>, level: usize, level_nodes: &mut Vec<Vec<i32>>) {
    if root.is_none() {
        return
    }
    if level_nodes.len() < level + 1 {
        level_nodes.push(vec![root.as_ref().unwrap().borrow().val]);
    } else {
        level_nodes[level][0] = root.as_ref().unwrap().borrow().val;
    }

    let root = Rc::clone(root.as_ref().unwrap());
    let root = root.borrow();
    myimpl(root.left.clone(), level + 1, level_nodes);
    myimpl(root.right.clone(), level + 1, level_nodes);
}
