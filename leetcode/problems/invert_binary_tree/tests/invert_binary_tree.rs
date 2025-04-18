use invert_binary_tree::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Helper function to create a TreeNode wrapped in Rc<RefCell>
fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

// Helper function to build a tree from a level-order vector representation
fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() || vec[0].is_none() {
        return None;
    }

    let root = create_node(vec[0].unwrap());
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < vec.len() {
        if let Some(current_node_rc) = queue.pop_front() {
            let mut current_node = current_node_rc.as_ref().unwrap().borrow_mut();

            // Process left child
            if let Some(val) = vec[i] {
                let left_child = create_node(val);
                current_node.left = left_child.clone();
                queue.push_back(left_child);
            }
            i += 1;

            // Process right child
            if i < vec.len() {
                if let Some(val) = vec[i] {
                    let right_child = create_node(val);
                    current_node.right = right_child.clone();
                    queue.push_back(right_child);
                }
                i += 1;
            }
        } else {
             // Should not happen if vec representation is correct, but handles potential queue empty issue
            break;
        }
    }
    root
}

// Helper function to convert a tree back to a level-order vector representation
fn tree_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if root.is_none() {
        return result;
    }

    queue.push_back(root.clone());
    result.push(root.as_ref().map(|node| node.borrow().val));

    let mut nodes_in_level = VecDeque::new();
    if let Some(r) = root {
        nodes_in_level.push_back(r);
    }

    while !nodes_in_level.is_empty() {
        let node_rc = nodes_in_level.pop_front().unwrap();
        let node = node_rc.borrow();

        match &node.left {
            Some(left_child) => {
                result.push(Some(left_child.borrow().val));
                nodes_in_level.push_back(left_child.clone());
            }
            None => result.push(None),
        }

        match &node.right {
            Some(right_child) => {
                result.push(Some(right_child.borrow().val));
                nodes_in_level.push_back(right_child.clone());
            }
            None => result.push(None),
        }
    }

    // Trim trailing Nones for consistent comparison with LeetCode examples
    while result.last().map_or(false, |&x| x.is_none()) {
        result.pop();
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*; // Import helpers

    #[test]
    fn test_example_1() {
        let input_vec = vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)];
        let expected_vec = vec![Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)];
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }

    #[test]
    fn test_example_2() {
        let input_vec = vec![Some(2), Some(1), Some(3)];
        let expected_vec = vec![Some(2), Some(3), Some(1)];
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }

    #[test]
    fn test_example_3() {
        let input_vec: Vec<Option<i32>> = vec![];
        let expected_vec: Vec<Option<i32>> = vec![];
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }

    #[test]
    fn test_single_node() {
        let input_vec = vec![Some(1)];
        let expected_vec = vec![Some(1)];
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }

     #[test]
    fn test_only_left_children() {
        // Represents [1, 2, null, 3]
        let input_vec = vec![Some(1), Some(2), None, Some(3)]; 
        // Expected: [1, null, 2, null, 3]
        let expected_vec = vec![Some(1), None, Some(2), None, Some(3)];
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }

     #[test]
    fn test_only_right_children() {
        // Represents [1, null, 2, null, 3]
        let input_vec = vec![Some(1), None, Some(2), None, Some(3)];
        // Expected: [1, 3, null, 2]
        let expected_vec = vec![Some(1), Some(3), None, Some(2)]; 
        let root = vec_to_tree(input_vec);
        let inverted_root = Solution::invert_tree(root);
        assert_eq!(tree_to_vec(inverted_root), expected_vec);
    }
}
