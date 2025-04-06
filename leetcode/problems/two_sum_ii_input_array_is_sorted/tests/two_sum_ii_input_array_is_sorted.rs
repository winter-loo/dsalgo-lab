use two_sum_ii_input_array_is_sorted::Solution;

#[test]
fn test_example_1() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
}

#[test]
fn test_example_2() {
    let numbers = vec![2, 3, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(numbers, target), vec![1, 3]);
}

#[test]
fn test_example_3() {
    let numbers = vec![-1, 0];
    let target = -1;
    assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
}

#[test]
fn test_larger_array() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 15;
    assert_eq!(Solution::two_sum(numbers, target), vec![5, 10]);
}

#[test]
fn test_target_at_extremes() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 11;
    assert_eq!(Solution::two_sum(numbers, target), vec![1, 10]);
}
