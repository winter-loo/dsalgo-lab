use missing_number::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [3,0,1]
    // Output: 2
    let nums = vec![3, 0, 1];
    assert_eq!(Solution::missing_number(nums), 2);
}

#[test]
fn test_example_2() {
    // Input: nums = [0,1]
    // Output: 2
    let nums = vec![0, 1];
    assert_eq!(Solution::missing_number(nums), 2);
}

#[test]
fn test_example_3() {
    // Input: nums = [9,6,4,2,3,5,7,0,1]
    // Output: 8
    let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    assert_eq!(Solution::missing_number(nums), 8);
}

#[test]
fn test_missing_zero() {
    // Test when 0 is the missing number
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::missing_number(nums), 0);
}

#[test]
fn test_missing_last() {
    // Test when the last number (n) is missing
    let nums = vec![0, 1, 2, 3];
    assert_eq!(Solution::missing_number(nums), 4);
}

#[test]
fn test_single_element() {
    // Test with a single element
    let nums = vec![0];
    assert_eq!(Solution::missing_number(nums), 1);
    
    let nums = vec![1];
    assert_eq!(Solution::missing_number(nums), 0);
}

#[test]
fn test_larger_array() {
    // Test with a larger array
    let mut nums: Vec<i32> = (0..100).collect();
    nums.remove(42); // Remove the number 42
    assert_eq!(Solution::missing_number(nums), 42);
}
