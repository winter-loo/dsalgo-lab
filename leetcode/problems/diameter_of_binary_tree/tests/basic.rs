use std::rc::Rc;
use std::cell::RefCell;
use diameter_of_binary_tree::{TreeNode, Solution};

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
    // Input: root = [1,2,3,4,5]
    // Output: 3
    let root = build_tree(&[Some(1), Some(2), Some(3), Some(4), Some(5)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}

#[test]
fn test_single_node() {
    // Input: root = [1]
    // Output: 0
    let root = build_tree(&[Some(1)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 0);
}

#[test]
fn test_left_skewed() {
    // Input: root = [1,2,None,3,None,4,None]
    // Output: 3
    let root = build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4)]);
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}
