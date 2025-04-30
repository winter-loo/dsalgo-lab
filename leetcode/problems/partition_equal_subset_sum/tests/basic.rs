use partition_equal_subset_sum::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [1,5,11,5]
    // Output: true
    // Explanation: The array can be partitioned as [1, 5, 5] and [11].
    let nums = vec![1, 5, 11, 5];
    assert_eq!(Solution::can_partition(nums), true);
}

#[test]
fn test_example_2() {
    // Input: nums = [1,2,3,5]
    // Output: false
    // Explanation: The array cannot be partitioned into equal sum subsets.
    let nums = vec![1, 2, 3, 5];
    assert_eq!(Solution::can_partition(nums), false);
}

#[test]
fn test_single_element() {
    // Single element array cannot be partitioned
    let nums = vec![1];
    assert_eq!(Solution::can_partition(nums), false);
}

#[test]
fn test_two_equal_elements() {
    // Two equal elements can be partitioned
    let nums = vec![1, 1];
    assert_eq!(Solution::can_partition(nums), true);
}

#[test]
fn test_odd_sum() {
    // Array with odd sum cannot be partitioned
    let nums = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::can_partition(nums), false);
}

#[test]
fn test_larger_example() {
    // Larger example that can be partitioned
    let nums = vec![3, 3, 3, 4, 5];
    assert_eq!(Solution::can_partition(nums), true);
}
