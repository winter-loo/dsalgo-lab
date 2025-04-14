use remove_nth_node_from_end_of_list::{ListNode, Solution};

// Helper function to convert a vector to a linked list
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut current = &mut dummy_head;
    
    for &val in vec.iter() {
        current.next = Some(Box::new(ListNode::new(val)));
        current = current.next.as_mut().unwrap();
    }
    
    dummy_head.next
}

// Helper function to convert a linked list to a vector
fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = list;
    
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    
    result
}

#[test]
fn test_example_1() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5]);
    let result = Solution::remove_nth_from_end(head, 2);
    assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
}

#[test]
fn test_example_2() {
    let head = vec_to_list(vec![1]);
    let result = Solution::remove_nth_from_end(head, 1);
    assert_eq!(list_to_vec(result), vec![]);
}

#[test]
fn test_example_3() {
    let head = vec_to_list(vec![1, 2]);
    let result = Solution::remove_nth_from_end(head, 1);
    assert_eq!(list_to_vec(result), vec![1]);
}

#[test]
fn test_remove_first_node() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5]);
    let result = Solution::remove_nth_from_end(head, 5);
    assert_eq!(list_to_vec(result), vec![2, 3, 4, 5]);
}

#[test]
fn test_remove_last_node() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5]);
    let result = Solution::remove_nth_from_end(head, 1);
    assert_eq!(list_to_vec(result), vec![1, 2, 3, 4]);
}

#[test]
fn test_longer_list() {
    let head = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let result = Solution::remove_nth_from_end(head, 4);
    assert_eq!(list_to_vec(result), vec![1, 2, 3, 4, 5, 6, 8, 9, 10]);
}
