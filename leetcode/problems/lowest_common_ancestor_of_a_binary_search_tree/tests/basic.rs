use std::rc::Rc;
use std::cell::RefCell;
use lowest_common_ancestor_of_a_binary_search_tree::{TreeNode, Solution};

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

fn find_node(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let node_val = node.borrow().val;
        if node_val == val {
            return Some(node);
        }
        if val < node_val {
            return find_node(node.borrow().left.clone(), val);
        } else {
            return find_node(node.borrow().right.clone(), val);
        }
    }
    None
}

#[test]
fn test_example_1() {
    // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
    // Output: 6
    let root = build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 8);
    let lca = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(lca.unwrap().borrow().val, 6);
}

#[test]
fn test_example_2() {
    // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
    // Output: 2
    let root = build_tree(&[Some(6), Some(2), Some(8), Some(0), Some(4), Some(7), Some(9), None, None, Some(3), Some(5)]);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 4);
    let lca = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(lca.unwrap().borrow().val, 2);
}

#[test]
fn test_example_3() {
    // Input: root = [2,1], p = 2, q = 1
    // Output: 2
    let root = build_tree(&[Some(2), Some(1)]);
    let p = find_node(root.clone(), 2);
    let q = find_node(root.clone(), 1);
    let lca = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(lca.unwrap().borrow().val, 2);
}
