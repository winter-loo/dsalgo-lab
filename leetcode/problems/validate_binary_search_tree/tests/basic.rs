use std::rc::Rc;
use std::cell::RefCell;
use validate_binary_search_tree::{TreeNode, Solution};

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
    // Input: root = [2,1,3]
    // Output: true
    let root = build_tree(&[Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::is_valid_bst(root), true);
}

#[test]
fn test_example_2() {
    // Input: root = [5,1,4,null,null,3,6]
    // Output: false
    let root = build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
    assert_eq!(Solution::is_valid_bst(root), false);
}

#[test]
fn test_equal_values() {
    // Input: root = [2,2,2]
    // Output: false
    let root = build_tree(&[Some(2), Some(2), Some(2)]);
    assert_eq!(Solution::is_valid_bst(root), false);
}

#[test]
fn test_right_subtree_violation() {
    // Input: root = [5,3,7,null,null,4,8]
    // Output: false
    let root = build_tree(&[Some(5), Some(3), Some(7), None, None, Some(4), Some(8)]);
    assert_eq!(Solution::is_valid_bst(root), false);
}

#[test]
fn test_valid_complex_tree() {
    // Input: root = [10,5,15,3,7,null,20]
    // Output: true
    let root = build_tree(&[Some(10), Some(5), Some(15), Some(3), Some(7), None, Some(20)]);
    assert_eq!(Solution::is_valid_bst(root), true);
}
