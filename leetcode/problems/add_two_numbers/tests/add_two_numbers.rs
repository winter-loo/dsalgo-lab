use add_two_numbers::{ListNode, Solution};

// Helper function to create a linked list from a vector (digits in reverse order)
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &val in vec.iter().rev() { // Iterate reversed vector to build list in correct order
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
        let l1 = vec_to_list(vec![2, 4, 3]);
        let l2 = vec_to_list(vec![5, 6, 4]);
        let expected = vec![7, 0, 8];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_example_2() {
        let l1 = vec_to_list(vec![0]);
        let l2 = vec_to_list(vec![0]);
        let expected = vec![0];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_example_3() {
        let l1 = vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = vec_to_list(vec![9, 9, 9, 9]);
        let expected = vec![8, 9, 9, 9, 0, 0, 0, 1];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_carry_at_end() {
        let l1 = vec_to_list(vec![5]);
        let l2 = vec_to_list(vec![5]);
        let expected = vec![0, 1];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }

    #[test]
    fn test_different_lengths_no_carry() {
        let l1 = vec_to_list(vec![1, 2]);
        let l2 = vec_to_list(vec![1, 2, 3]);
        let expected = vec![2, 4, 3];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }

     #[test]
    fn test_different_lengths_with_carry() {
        let l1 = vec_to_list(vec![1]);
        let l2 = vec_to_list(vec![9, 9]);
        let expected = vec![0, 0, 1];
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(list_to_vec(result), expected);
    }
}
