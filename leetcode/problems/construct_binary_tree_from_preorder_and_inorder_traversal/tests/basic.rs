use std::rc::Rc;
use std::cell::RefCell;
use construct_binary_tree_from_preorder_and_inorder_traversal::{TreeNode, Solution};

// Helper function to check if two trees are identical
fn is_same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p_borrow = p.borrow();
            let q_borrow = q.borrow();
            p_borrow.val == q_borrow.val
                && is_same_tree(&p_borrow.left, &q_borrow.left)
                && is_same_tree(&p_borrow.right, &q_borrow.right)
        },
        _ => false,
    }
}

// Helper function to build a tree from a level-order array representation
fn build_tree_from_level_order(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
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
    // Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
    // Output: [3,9,20,null,null,15,7]
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let expected = build_tree_from_level_order(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    let result = Solution::build_tree(preorder, inorder);
    assert!(is_same_tree(&result, &expected));
}

#[test]
fn test_example_2() {
    // Input: preorder = [-1], inorder = [-1]
    // Output: [-1]
    let preorder = vec![-1];
    let inorder = vec![-1];
    let expected = build_tree_from_level_order(&[Some(-1)]);
    let result = Solution::build_tree(preorder, inorder);
    assert!(is_same_tree(&result, &expected));
}

#[test]
fn test_left_skewed_tree() {
    // Input: preorder = [1,2,3], inorder = [3,2,1]
    // Output: [1,2,null,3]
    let preorder = vec![1, 2, 3];
    let inorder = vec![3, 2, 1];
    let expected = build_tree_from_level_order(&[Some(1), Some(2), None, Some(3)]);
    let result = Solution::build_tree(preorder, inorder);
    assert!(is_same_tree(&result, &expected));
}

#[test]
fn test_right_skewed_tree() {
    // Input: preorder = [1,2,3], inorder = [1,2,3]
    // Output: [1,null,2,null,3]
    let preorder = vec![1, 2, 3];
    let inorder = vec![1, 2, 3];
    let mut root = Rc::new(RefCell::new(TreeNode::new(1)));
    let mut current = root.clone();
    
    for i in 2..=3 {
        let new_node = Rc::new(RefCell::new(TreeNode::new(i)));
        current.borrow_mut().right = Some(new_node.clone());
        current = new_node;
    }
    
    let result = Solution::build_tree(preorder, inorder);
    assert!(is_same_tree(&result, &Some(root)));
}
