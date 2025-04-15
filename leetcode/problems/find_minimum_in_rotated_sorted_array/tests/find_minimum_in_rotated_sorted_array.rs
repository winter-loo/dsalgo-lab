use find_minimum_in_rotated_sorted_array::Solution;

#[test]
fn test_example_1() {
    let nums = vec![3, 4, 5, 1, 2];
    assert_eq!(Solution::find_min(nums), 1);
}

#[test]
fn test_example_2() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(Solution::find_min(nums), 0);
}

#[test]
fn test_example_3() {
    let nums = vec![11, 13, 15, 17];
    assert_eq!(Solution::find_min(nums), 11);
}

#[test]
fn test_single_element() {
    let nums = vec![5];
    assert_eq!(Solution::find_min(nums), 5);
}

#[test]
fn test_two_elements_rotated() {
    let nums = vec![2, 1];
    assert_eq!(Solution::find_min(nums), 1);
}

#[test]
fn test_two_elements_not_rotated() {
    let nums = vec![1, 2];
    assert_eq!(Solution::find_min(nums), 1);
}

#[test]
fn test_rotated_once() {
    let nums = vec![7, 0, 1, 2, 3, 4, 5, 6];
    assert_eq!(Solution::find_min(nums), 0);
}

#[test]
fn test_not_rotated() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(Solution::find_min(nums), 0);
}
