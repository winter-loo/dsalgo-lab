use merge_k_sorted_lists::{ListNode, Solution};

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
        let lists = vec![
            vec_to_list(vec![1, 4, 5]),
            vec_to_list(vec![1, 3, 4]),
            vec_to_list(vec![2, 6]),
        ];
        let expected = vec![1, 1, 2, 3, 4, 4, 5, 6];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_example_2() {
        let lists: Vec<Option<Box<ListNode>>> = vec![];
        let expected: Vec<i32> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_example_3() {
        let lists = vec![vec_to_list(vec![])];
        let expected: Vec<i32> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_one_list() {
        let lists = vec![vec_to_list(vec![1, 2, 3])];
        let expected = vec![1, 2, 3];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_empty_lists_mixed() {
        let lists = vec![
            vec_to_list(vec![1, 3]),
            vec_to_list(vec![]),
            vec_to_list(vec![2]),
        ];
        let expected = vec![1, 2, 3];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_all_empty_lists() {
        let lists = vec![
            vec_to_list(vec![]),
            vec_to_list(vec![]),
            vec_to_list(vec![]),
        ];
        let expected: Vec<i32> = vec![];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(result), expected);
    }
}
