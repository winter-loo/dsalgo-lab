use search_in_rotated_sorted_array::Solution;

#[test]
fn test_example_1() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(Solution::search(nums, target), 4);
}

#[test]
fn test_example_2() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    assert_eq!(Solution::search(nums, target), -1);
}

#[test]
fn test_example_3() {
    let nums = vec![1];
    let target = 0;
    assert_eq!(Solution::search(nums, target), -1);
}

#[test]
fn test_single_element_found() {
    let nums = vec![5];
    let target = 5;
    assert_eq!(Solution::search(nums, target), 0);
}

#[test]
fn test_two_elements_rotated_found() {
    let nums = vec![2, 1];
    let target = 1;
    assert_eq!(Solution::search(nums, target), 1);
}

#[test]
fn test_two_elements_not_rotated_found() {
    let nums = vec![1, 2];
    let target = 2;
    assert_eq!(Solution::search(nums, target), 1);
}

#[test]
fn test_target_in_left_sorted_portion() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 5;
    assert_eq!(Solution::search(nums, target), 1);
}

#[test]
fn test_target_in_right_sorted_portion() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 1;
    assert_eq!(Solution::search(nums, target), 5);
}

#[test]
fn test_not_rotated() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let target = 4;
    assert_eq!(Solution::search(nums, target), 4);
}

#[test]
fn test_rotated_once() {
    let nums = vec![7, 0, 1, 2, 3, 4, 5, 6];
    let target = 7;
    assert_eq!(Solution::search(nums, target), 0);
}
