use std::rc::Rc;
use std::cell::RefCell;
use subtree_of_another_tree::{TreeNode, Solution};

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
    // Input: root = [3,4,5,1,2], subRoot = [4,1,2]
    // Output: true
    let root = build_tree(&[Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let subroot = build_tree(&[Some(4), Some(1), Some(2)]);
    assert_eq!(Solution::is_subtree(root, subroot), true);
}

#[test]
fn test_example_2() {
    // Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
    // Output: false
    let root = build_tree(&[Some(3), Some(4), Some(5), Some(1), Some(2), None, None, None, None, Some(0)]);
    let subroot = build_tree(&[Some(4), Some(1), Some(2)]);
    assert_eq!(Solution::is_subtree(root, subroot), false);
}

#[test]
fn test_identical_trees() {
    // Input: root = [1,2,3], subRoot = [1,2,3]
    // Output: true
    let root = build_tree(&[Some(1), Some(2), Some(3)]);
    let subroot = build_tree(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::is_subtree(root, subroot), true);
}

#[test]
fn test_subroot_empty() {
    // Input: root = [1,2,3], subRoot = []
    // Output: true
    let root = build_tree(&[Some(1), Some(2), Some(3)]);
    let subroot = build_tree(&[]);
    assert_eq!(Solution::is_subtree(root, subroot), true);
}
