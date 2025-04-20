use std::rc::Rc;
use std::cell::RefCell;
use count_good_nodes_in_binary_tree::{TreeNode, Solution};

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
    // Input: root = [3,1,4,3,null,1,5]
    // Output: 4
    let root = build_tree(&[Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)]);
    assert_eq!(Solution::good_nodes(root), 4);
}

#[test]
fn test_example_2() {
    // Input: root = [3,3,null,4,2]
    // Output: 3
    let root = build_tree(&[Some(3), Some(3), None, Some(4), Some(2)]);
    assert_eq!(Solution::good_nodes(root), 3);
}

#[test]
fn test_example_3() {
    // Input: root = [1]
    // Output: 1
    let root = build_tree(&[Some(1)]);
    assert_eq!(Solution::good_nodes(root), 1);
}

#[test]
fn test_decreasing_path() {
    // Input: root = [5,4,3,2,1]
    // Output: 1
    let root = build_tree(&[Some(5), Some(4), Some(3), Some(2), Some(1)]);
    assert_eq!(Solution::good_nodes(root), 1);
}

#[test]
fn test_increasing_path() {
    // Input: root = [1,2,3,4,5]
    // Output: 5
    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::good_nodes(root), 5);
}
