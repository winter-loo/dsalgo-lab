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

use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut myq = VecDeque::new();
        myq.push_back(root.unwrap());
        let mut result = vec![];

        while !myq.is_empty() {
            let level_size = myq.len();

            let mut level_values = vec![];
            for _ in 0..level_size {
                if let Some(node) = myq.pop_front() {
                    level_values.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.as_ref() {
                        myq.push_back(Rc::clone(left));
                    }
                    if let Some(right) = node.borrow().right.as_ref() {
                        myq.push_back(Rc::clone(right));
                    }
                }
            }
            result.push(level_values);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    // Helper function to create a TreeNode wrapped in Option<Rc<RefCell<...>>>
    fn new_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn example_1() {
        // Input: root = [3,9,20,null,null,15,7]
        let root = new_node(3).unwrap();
        root.borrow_mut().left = new_node(9);
        let right = new_node(20).unwrap();
        right.borrow_mut().left = new_node(15);
        right.borrow_mut().right = new_node(7);
        root.borrow_mut().right = Some(right);

        let expected = vec![
            vec![3],
            vec![9, 20],
            vec![15, 7],
        ];
        assert_eq!(Solution::level_order(Some(root)), expected);
    }

    #[test]
    fn example_2() {
        // Input: root = [1]
        let root = new_node(1);
        let expected = vec![
            vec![1],
        ];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn example_3() {
        // Input: root = []
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
    }

     #[test]
    fn single_node() {
        let root = new_node(5);
        let expected = vec![vec![5]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn perfect_tree() {
        //     1
        //    / \
        //   2   3
        //  / \ / \
        // 4  5 6  7
        let root = new_node(1).unwrap();
        let left1 = new_node(2).unwrap();
        let right1 = new_node(3).unwrap();
        left1.borrow_mut().left = new_node(4);
        left1.borrow_mut().right = new_node(5);
        right1.borrow_mut().left = new_node(6);
        right1.borrow_mut().right = new_node(7);
        root.borrow_mut().left = Some(left1);
        root.borrow_mut().right = Some(right1);

        let expected = vec![
            vec![1],
            vec![2, 3],
            vec![4, 5, 6, 7],
        ];
        assert_eq!(Solution::level_order(Some(root)), expected);
    }
}
