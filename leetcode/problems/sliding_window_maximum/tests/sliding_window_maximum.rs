use sliding_window_maximum::Solution;

#[test]
fn test_example_1() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let expected = vec![3, 3, 5, 5, 6, 7];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_example_2() {
    let nums = vec![1];
    let k = 1;
    let expected = vec![1];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_large_window() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 8;
    let expected = vec![7];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_decreasing_array() {
    let nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    let k = 3;
    let expected = vec![9, 8, 7, 6, 5, 4, 3];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_increasing_array() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let k = 3;
    let expected = vec![3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_with_duplicates() {
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let k = 3;
    let expected = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}

#[test]
fn test_example_3() {
    let nums = vec![9, 10, 9, -7, -4, -8, 2, -6];
    let k = 5;
    let expected = vec![10, 10, 9, 2];
    assert_eq!(Solution::max_sliding_window(nums, k), expected);
}
