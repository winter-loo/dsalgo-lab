use kth_largest_element_in_a_stream::KthLargest;

#[test]
fn test_example_1() {
    // ["KthLargest", "add", "add", "add", "add", "add"]
    // [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
    // Output: [null, 4, 5, 5, 8, 8]
    let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
    assert_eq!(kth_largest.add(3), 4);
    assert_eq!(kth_largest.add(5), 5);
    assert_eq!(kth_largest.add(10), 5);
    assert_eq!(kth_largest.add(9), 8);
    assert_eq!(kth_largest.add(4), 8);
}

#[test]
fn test_empty_initial_array() {
    // Test with an empty initial array
    let mut kth_largest = KthLargest::new(1, vec![]);
    assert_eq!(kth_largest.add(3), 3);
    assert_eq!(kth_largest.add(5), 5);
    assert_eq!(kth_largest.add(10), 10);
    assert_eq!(kth_largest.add(9), 10);
    assert_eq!(kth_largest.add(4), 10);
}

#[test]
fn test_k_equals_array_length() {
    // Test when k equals the length of the initial array
    let mut kth_largest = KthLargest::new(3, vec![4, 5, 8]);
    assert_eq!(kth_largest.add(3), 3);
    assert_eq!(kth_largest.add(5), 4);
    assert_eq!(kth_largest.add(10), 5);
}

#[test]
fn test_negative_numbers() {
    // Test with negative numbers
    let mut kth_largest = KthLargest::new(2, vec![-5, -10]);
    assert_eq!(kth_largest.add(-1), -5);
    assert_eq!(kth_largest.add(-2), -2);
    assert_eq!(kth_largest.add(-6), -2);
    assert_eq!(kth_largest.add(0), -1);
}
