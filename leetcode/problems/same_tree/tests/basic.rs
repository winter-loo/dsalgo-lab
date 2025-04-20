use std::rc::Rc;
use std::cell::RefCell;
use same_tree::{TreeNode, Solution};

fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    if nodes.is_empty() || nodes[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(nodes[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < nodes.len() {
        let current = queue.pop_front().unwrap();
        if let Some(val) = nodes[i] {
            let left = Rc::new(RefCell::new(TreeNode::new(val)));
            current.borrow_mut().left = Some(left.clone());
            queue.push_back(left);
        }
        i += 1;
        if i < nodes.len() {
            if let Some(val) = nodes[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }
    Some(root)
}

#[test]
fn test_example_1() {
    // Input: p = [1,2,3], q = [1,2,3]
    // Output: true
    let p = build_tree(&[Some(1), Some(2), Some(3)]);
    let q = build_tree(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::is_same_tree(p, q), true);
}

#[test]
fn test_example_2() {
    // Input: p = [1,2], q = [1,null,2]
    // Output: false
    let p = build_tree(&[Some(1), Some(2)]);
    let q = build_tree(&[Some(1), None, Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}

#[test]
fn test_example_3() {
    // Input: p = [1,2,1], q = [1,1,2]
    // Output: false
    let p = build_tree(&[Some(1), Some(2), Some(1)]);
    let q = build_tree(&[Some(1), Some(1), Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}

#[test]
fn test_both_empty() {
    // Input: p = [], q = []
    // Output: true
    let p = build_tree(&[]);
    let q = build_tree(&[]);
    assert_eq!(Solution::is_same_tree(p, q), true);
}

#[test]
fn test_one_empty() {
    // Input: p = [1], q = []
    // Output: false
    let p = build_tree(&[Some(1)]);
    let q = build_tree(&[]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}
