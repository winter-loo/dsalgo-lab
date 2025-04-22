use kth_largest_element_in_an_array::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [3,2,1,5,6,4], k = 2
    // Output: 5
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    assert_eq!(Solution::find_kth_largest(nums, k), 5);
}

#[test]
fn test_example_2() {
    // Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
    // Output: 4
    let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k = 4;
    assert_eq!(Solution::find_kth_largest(nums, k), 4);
}

#[test]
fn test_single_element() {
    // Input: nums = [1], k = 1
    // Output: 1
    let nums = vec![1];
    let k = 1;
    assert_eq!(Solution::find_kth_largest(nums, k), 1);
}

#[test]
fn test_all_same_elements() {
    // Input: nums = [3,3,3,3,3], k = 2
    // Output: 3
    let nums = vec![3, 3, 3, 3, 3];
    let k = 2;
    assert_eq!(Solution::find_kth_largest(nums, k), 3);
}

#[test]
fn test_k_equals_length() {
    // Input: nums = [1,2,3,4,5], k = 5
    // Output: 1
    let nums = vec![1, 2, 3, 4, 5];
    let k = 5;
    assert_eq!(Solution::find_kth_largest(nums, k), 1);
}
