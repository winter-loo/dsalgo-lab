use std::rc::Rc;
use std::cell::RefCell;
use kth_smallest_element_in_a_bst::{TreeNode, Solution};

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
    // Input: root = [3,1,4,null,2], k = 1
    // Output: 1
    let root = build_tree(&[Some(3), Some(1), Some(4), None, Some(2)]);
    assert_eq!(Solution::kth_smallest(root, 1), 1);
}

#[test]
fn test_example_2() {
    // Input: root = [5,3,6,2,4,null,null,1], k = 3
    // Output: 3
    let root = build_tree(&[Some(5), Some(3), Some(6), Some(2), Some(4), None, None, Some(1)]);
    assert_eq!(Solution::kth_smallest(root, 3), 3);
}

#[test]
fn test_single_node() {
    // Input: root = [1], k = 1
    // Output: 1
    let root = build_tree(&[Some(1)]);
    assert_eq!(Solution::kth_smallest(root, 1), 1);
}

#[test]
fn test_balanced_tree() {
    // Input: root = [4,2,6,1,3,5,7], k = 5
    // Output: 5
    let root = build_tree(&[Some(4), Some(2), Some(6), Some(1), Some(3), Some(5), Some(7)]);
    assert_eq!(Solution::kth_smallest(root, 5), 5);
}

#[test]
fn test_right_skewed() {
    // Input: root = [1,null,2,null,3,null,4], k = 4
    // Output: 4
    let mut root = Rc::new(RefCell::new(TreeNode::new(1)));
    let mut current = root.clone();
    
    for i in 2..=4 {
        let new_node = Rc::new(RefCell::new(TreeNode::new(i)));
        current.borrow_mut().right = Some(new_node.clone());
        current = new_node;
    }
    
    assert_eq!(Solution::kth_smallest(Some(root), 4), 4);
}
