use single_number::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [2,2,1]
    // Output: 1
    let nums = vec![2, 2, 1];
    assert_eq!(Solution::single_number(nums), 1);
}

#[test]
fn test_example_2() {
    // Input: nums = [4,1,2,1,2]
    // Output: 4
    let nums = vec![4, 1, 2, 1, 2];
    assert_eq!(Solution::single_number(nums), 4);
}

#[test]
fn test_example_3() {
    // Input: nums = [1]
    // Output: 1
    let nums = vec![1];
    assert_eq!(Solution::single_number(nums), 1);
}

#[test]
fn test_negative_numbers() {
    // Test with negative numbers
    let nums = vec![-1, -1, -2];
    assert_eq!(Solution::single_number(nums), -2);
}

#[test]
fn test_mixed_numbers() {
    // Test with mixed positive and negative numbers
    let nums = vec![-1, 2, 3, -1, 2];
    assert_eq!(Solution::single_number(nums), 3);
}

#[test]
fn test_larger_array() {
    // Test with a larger array
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(Solution::single_number(nums), 9);
}

#[test]
fn test_zero() {
    // Test with zero as the single number
    let nums = vec![1, 1, 0, 2, 2];
    assert_eq!(Solution::single_number(nums), 0);
}
