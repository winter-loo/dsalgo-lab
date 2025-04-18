use maximum_depth_of_binary_tree::{Solution, TreeNode};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// --- Copied Helper Functions --- 

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
             // Should not happen if vec representation is correct
             break;
        }
    }
    root
}

// --- Test Module --- 

#[cfg(test)]
mod tests {
    use super::*; // Import helpers

    #[test]
    fn test_example_1() {
        let input_vec = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let expected_depth = 3;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_example_2() {
        let input_vec = vec![Some(1), None, Some(2)];
        let expected_depth = 2;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_empty_tree() {
        let input_vec: Vec<Option<i32>> = vec![];
        let expected_depth = 0;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_single_node() {
        let input_vec = vec![Some(0)];
        let expected_depth = 1;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_left_skewed_tree() {
        let input_vec = vec![Some(1), Some(2), None, Some(3), None, Some(4)];
        let expected_depth = 4;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_right_skewed_tree() {
        let input_vec = vec![Some(1), None, Some(2), None, Some(3), None, Some(4)];
        let expected_depth = 4;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }

    #[test]
    fn test_perfect_binary_tree() {
        let input_vec = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
        let expected_depth = 3;
        let root = vec_to_tree(input_vec);
        assert_eq!(Solution::max_depth(root), expected_depth);
    }
}
