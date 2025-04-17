use reorder_list::{ListNode, Solution};

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
    let mut head = vec_to_list(vec![1, 2, 3, 4]);
    Solution::reorder_list(&mut head);
    println!("head={head:?}");
    assert_eq!(list_to_vec(head), vec![1, 4, 2, 3]);
}

#[test]
fn test_example_2() {
    let mut head = vec_to_list(vec![1, 2, 3, 4, 5]);
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![1, 5, 2, 4, 3]);
}

#[test]
fn test_single_node() {
    let mut head = vec_to_list(vec![1]);
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![1]);
}

#[test]
fn test_two_nodes() {
    let mut head = vec_to_list(vec![1, 2]);
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![1, 2]);
}

#[test]
fn test_three_nodes() {
    let mut head = vec_to_list(vec![1, 2, 3]);
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![1, 3, 2]);
}

#[test]
fn test_empty_list() {
    let mut head: Option<Box<ListNode>> = None;
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![]);
}

#[test]
fn test_longer_list() {
    let mut head = vec_to_list(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    Solution::reorder_list(&mut head);
    assert_eq!(list_to_vec(head), vec![1, 8, 2, 7, 3, 6, 4, 5]);
}
