use merge_intervals::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
    // Output: [[1,6],[8,10],[15,18]]
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[1,4],[4,5]]
    // Output: [[1,5]]
    let intervals = vec![vec![1, 4], vec![4, 5]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
}

#[test]
fn test_single_interval() {
    // Test with a single interval
    let intervals = vec![vec![1, 3]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 3]]);
}

#[test]
fn test_non_overlapping() {
    // Test with non-overlapping intervals
    let intervals = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
}

#[test]
fn test_all_overlapping() {
    // Test with all overlapping intervals
    let intervals = vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 10]]);
}

#[test]
fn test_unsorted_intervals() {
    // Test with unsorted intervals
    let intervals = vec![vec![3, 6], vec![1, 3], vec![8, 10], vec![15, 18]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
}

#[test]
fn test_nested_intervals() {
    // Test with nested intervals
    let intervals = vec![vec![1, 6], vec![2, 5], vec![3, 4]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 6]]);
}

#[test]
fn test_adjacent_intervals() {
    // Test with adjacent intervals
    let intervals = vec![vec![1, 3], vec![3, 6], vec![6, 9]];
    assert_eq!(Solution::merge(intervals), vec![vec![1, 9]]);
}
