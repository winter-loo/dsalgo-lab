use reverse_nodes_in_k_group::{ListNode, Solution};

// Helper function to create a linked list from a vector
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = current;
        current = Some(Box::new(new_node));
    }
    current
}

// Helper function to convert a linked list back to a vector for comparison
fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        vec.push(node.val);
        current = node.next;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*; // Import helpers from parent module

    #[test]
    fn test_example_1() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let k = 2;
        let expected = vec![2, 1, 4, 3, 5];
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_example_2() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let k = 3;
        let expected = vec![3, 2, 1, 4, 5];
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_k_equals_one() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let k = 1;
        let expected = vec![1, 2, 3, 4, 5]; // Should not change
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_k_equals_length() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5]);
        let k = 5;
        let expected = vec![5, 4, 3, 2, 1];
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_length_not_multiple_of_k() {
        let head = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7]);
        let k = 3;
        let expected = vec![3, 2, 1, 6, 5, 4, 7]; // Last node remains
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }
    
    #[test]
    fn test_empty_list() {
        let head: Option<Box<ListNode>> = None;
        let k = 3;
        let expected: Vec<i32> = vec![];
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_fewer_than_k_nodes() {
        let head = vec_to_list(vec![1, 2]);
        let k = 3;
        let expected = vec![1, 2]; // Should not change
        let result = Solution::reverse_k_group(head, k);
        assert_eq!(list_to_vec(result), expected);
    }
}
