use largest_rectangle_in_histogram::Solution;

#[test]
fn test_example_1() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    // [1, 5, 6]
    assert_eq!(Solution::largest_rectangle_area(heights), 10);
}

#[test]
fn test_example_2() {
    let heights = vec![2, 4];
    assert_eq!(Solution::largest_rectangle_area(heights), 4);
}

#[test]
fn test_empty() {
    let heights = vec![];
    assert_eq!(Solution::largest_rectangle_area(heights), 0);
}

#[test]
fn test_single_bar() {
    let heights = vec![5];
    assert_eq!(Solution::largest_rectangle_area(heights), 5);
}

#[test]
fn test_all_same_height() {
    let heights = vec![3, 3, 3, 3];
    assert_eq!(Solution::largest_rectangle_area(heights), 12);
}

#[test]
fn test_ascending() {
    let heights = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::largest_rectangle_area(heights), 9);
}

#[test]
fn test_descending() {
    let heights = vec![5, 4, 3, 2, 1];
    assert_eq!(Solution::largest_rectangle_area(heights), 9);
}

#[test]
fn test_example_3() {
    let heights = vec![1,2,3,2,3,2];
    assert_eq!(Solution::largest_rectangle_area_partition(heights), 10);
}