use merge_two_sorted_lists::{ListNode, Solution};

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
    let list1 = vec_to_list(vec![1, 2, 4]);
    let list2 = vec_to_list(vec![1, 3, 4]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
}

#[test]
fn test_example_2() {
    let list1 = vec_to_list(vec![]);
    let list2 = vec_to_list(vec![]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![]);
}

#[test]
fn test_example_3() {
    let list1 = vec_to_list(vec![]);
    let list2 = vec_to_list(vec![0]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![0]);
}

#[test]
fn test_first_list_empty() {
    let list1 = vec_to_list(vec![]);
    let list2 = vec_to_list(vec![1, 2, 3]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![1, 2, 3]);
}

#[test]
fn test_second_list_empty() {
    let list1 = vec_to_list(vec![1, 2, 3]);
    let list2 = vec_to_list(vec![]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![1, 2, 3]);
}

#[test]
fn test_interleaved_lists() {
    let list1 = vec_to_list(vec![1, 3, 5]);
    let list2 = vec_to_list(vec![2, 4, 6]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_one_list_smaller() {
    let list1 = vec_to_list(vec![1, 2, 3]);
    let list2 = vec_to_list(vec![4, 5, 6]);
    let merged = Solution::merge_two_lists(list1, list2);
    assert_eq!(list_to_vec(merged), vec![1, 2, 3, 4, 5, 6]);
}
