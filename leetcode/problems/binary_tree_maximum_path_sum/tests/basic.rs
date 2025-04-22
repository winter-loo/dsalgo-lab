use std::rc::Rc;
use std::cell::RefCell;
use binary_tree_maximum_path_sum::{TreeNode, Solution};

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
    // Input: root = [1,2,3]
    // Output: 6
    let root = build_tree(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::max_path_sum(root), 6);
}

#[test]
fn test_example_2() {
    // Input: root = [-10,9,20,null,null,15,7]
    // Output: 42
    let root = build_tree(&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::max_path_sum(root), 42);
}

#[test]
fn test_single_node() {
    // Input: root = [5]
    // Output: 5
    let root = build_tree(&[Some(5)]);
    assert_eq!(Solution::max_path_sum(root), 5);
}

#[test]
fn test_negative_values() {
    // Input: root = [-3]
    // Output: -3
    let root = build_tree(&[Some(-3)]);
    assert_eq!(Solution::max_path_sum(root), -3);
}

#[test]
fn test_complex_tree() {
    // Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1]
    // Output: 48 (path: 5 -> 8 -> 13 -> 4 -> 1 = 31, or 11 -> 7 -> 2 = 20, or other paths)
    let root = build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)]);
    assert_eq!(Solution::max_path_sum(root), 48);
}
