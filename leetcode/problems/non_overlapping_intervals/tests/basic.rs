use non_overlapping_intervals::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
    // Output: 1
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 1);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[1,2],[1,2],[1,2]]
    // Output: 2
    let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
}

#[test]
fn test_example_3() {
    // Input: intervals = [[1,2],[2,3]]
    // Output: 0
    let intervals = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
}

#[test]
fn test_single_interval() {
    // Test with a single interval
    let intervals = vec![vec![1, 2]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
}

#[test]
fn test_all_overlapping() {
    // Test with all overlapping intervals
    let intervals = vec![vec![1, 10], vec![2, 9], vec![3, 8], vec![4, 7]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 3);
}

#[test]
fn test_unsorted_intervals() {
    // Test with unsorted intervals
    let intervals = vec![vec![3, 4], vec![1, 2], vec![2, 3], vec![1, 3]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 1);
}

#[test]
fn test_nested_intervals() {
    // Test with nested intervals
    let intervals = vec![vec![1, 6], vec![2, 5], vec![3, 4]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
}

#[test]
fn test_adjacent_intervals() {
    // Test with adjacent intervals
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
}
