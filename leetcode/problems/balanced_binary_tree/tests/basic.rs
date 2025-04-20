use std::rc::Rc;
use std::cell::RefCell;
use balanced_binary_tree::{TreeNode, Solution};

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
    // Input: root = [3,9,20,null,null,15,7]
    // Output: true
    let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::is_balanced(root), true);
}

#[test]
fn test_example_2() {
    // Input: root = [1,2,2,3,3,null,null,4,4]
    // Output: false
    let root = build_tree(&[Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)]);
    assert_eq!(Solution::is_balanced(root), false);
}

#[test]
fn test_empty() {
    // Input: root = []
    // Output: true
    let root = build_tree(&[]);
    assert_eq!(Solution::is_balanced(root), true);
}
