use binary_search::Solution;

#[test]
fn test_example_1() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    assert_eq!(Solution::search(nums, target), 4);
}

#[test]
fn test_example_2() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    assert_eq!(Solution::search(nums, target), -1);
}

#[test]
fn test_empty_array() {
    let nums = vec![];
    let target = 5;
    assert_eq!(Solution::search(nums, target), -1);
}

#[test]
fn test_single_element_found() {
    let nums = vec![5];
    let target = 5;
    assert_eq!(Solution::search(nums, target), 0);
}

#[test]
fn test_single_element_not_found() {
    let nums = vec![5];
    let target = 1;
    assert_eq!(Solution::search(nums, target), -1);
}

#[test]
fn test_first_element() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 1;
    assert_eq!(Solution::search(nums, target), 0);
}

#[test]
fn test_last_element() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 5;
    assert_eq!(Solution::search(nums, target), 4);
}
