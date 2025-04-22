use std::rc::Rc;
use std::cell::RefCell;
use serialize_and_deserialize_binary_tree::{TreeNode, Codec};

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
    // Input: root = [1,2,3,null,null,4,5]
    let root = build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
    let codec = Codec::new();
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized);
    assert!(is_same_tree(&root, &deserialized));
}

#[test]
fn test_example_2() {
    // Input: root = []
    let root = build_tree(&[]);
    let codec = Codec::new();
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized);
    assert!(is_same_tree(&root, &deserialized));
}

#[test]
fn test_single_node() {
    // Input: root = [5]
    let root = build_tree(&[Some(5)]);
    let codec = Codec::new();
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized);
    assert!(is_same_tree(&root, &deserialized));
}

#[test]
fn test_complex_tree() {
    // Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1]
    let root = build_tree(&[Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)]);
    let codec = Codec::new();
    let serialized = codec.serialize(root.clone());
    let deserialized = codec.deserialize(serialized);
    assert!(is_same_tree(&root, &deserialized));
}
