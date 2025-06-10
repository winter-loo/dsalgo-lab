use target_sum::Solution;

#[test]
fn test_example_1() {
    // Input: nums = [1,1,1,1,1], target = 3
    // Output: 5
    let nums = vec![1, 1, 1, 1, 1];
    let target = 3;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 5);
}

#[test]
fn test_example_2() {
    // Input: nums = [1], target = 1
    // Output: 1
    let nums = vec![1];
    let target = 1;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 1);
}

#[test]
fn test_impossible_target() {
    // Target that cannot be achieved
    let nums = vec![1, 2, 3];
    let target = 7;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 0);
}

#[test]
fn test_zero_target() {
    // Target of 0
    let nums = vec![1, 1, 1, 1];
    let target = 0;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 6);
}

#[test]
fn test_negative_target() {
    // Negative target
    let nums = vec![1, 2, 1];
    let target = -2;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 2);
}

#[test]
fn test_larger_numbers() {
    // Test with larger numbers
    let nums = vec![25, 18, 47, 13, 45];
    let target = 42;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 0);
}

#[test]
fn test_example_3() {
    // Test with larger numbers
    let nums = vec![1, 0];
    let target = 1;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 2);
}

#[test]
fn test_example_4() {
    // Test with larger numbers
    let nums = vec![
        2, 107, 109, 113, 127, 131, 137, 3, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 47, 53,
    ];
    let target = 1000;
    assert_eq!(Solution::find_target_sum_ways(nums, target), 0);
}
