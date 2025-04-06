use two_sum::Solution;

#[test]
fn test_example_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![0, 1]) || (result == vec![1, 0]),
        "Expected [0, 1] or [1, 0], got {:?}",
        result
    );
}

#[test]
fn test_example_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![1, 2]) || (result == vec![2, 1]),
        "Expected [1, 2] or [2, 1], got {:?}",
        result
    );
}

#[test]
fn test_example_3() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![0, 1]) || (result == vec![1, 0]),
        "Expected [0, 1] or [1, 0], got {:?}",
        result
    );
}

#[test]
fn test_negative_numbers() {
    let nums = vec![-1, -2, -3, -4, -5];
    let target = -8;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![2, 4]) || (result == vec![4, 2]),
        "Expected [2, 4] or [4, 2], got {:?}",
        result
    );
}

#[test]
fn test_mixed_numbers() {
    let nums = vec![2, -7, 11, -15];
    let target = -5;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![0, 1]) || (result == vec![1, 0]),
        "Expected [0, 1] or [1, 0], got {:?}",
        result
    );
}

#[test]
fn test_larger_array() {
    let nums = vec![1, 3, 5, 7, 9, 11, 13, 15, 17];
    let target = 16;
    let result = Solution::two_sum(nums, target);
    assert!(
        (result == vec![1, 6]) || (result == vec![6, 1]),
        "Expected [1, 6] or [6, 1], got {:?}",
        result
    );
}
