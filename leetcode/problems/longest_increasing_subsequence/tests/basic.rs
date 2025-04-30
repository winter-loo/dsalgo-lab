use longest_increasing_subsequence::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [10,9,2,5,3,7,101,18]
    // Output: 4
    // Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(Solution::length_of_lis(nums), 4);
}

#[test]
fn test_example_2() {
    // Input: nums = [0,1,0,3,2,3]
    // Output: 4
    let nums = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(Solution::length_of_lis(nums), 4);
}

#[test]
fn test_example_3() {
    // Input: nums = [7,7,7,7,7,7,7]
    // Output: 1
    let nums = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(Solution::length_of_lis(nums), 1);
}

#[test]
fn test_single_element() {
    // Single element array
    let nums = vec![5];
    assert_eq!(Solution::length_of_lis(nums), 1);
}

#[test]
fn test_strictly_decreasing() {
    // Strictly decreasing array
    let nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    assert_eq!(Solution::length_of_lis(nums), 1);
}

#[test]
fn test_strictly_increasing() {
    // Strictly increasing array
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::length_of_lis(nums), 9);
}
