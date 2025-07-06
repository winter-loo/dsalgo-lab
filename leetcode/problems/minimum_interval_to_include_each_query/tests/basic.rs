use minimum_interval_to_include_each_query::Solution;

#[test]
fn test_example_1() {
    // Input: intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]
    // Output: [3,3,1,4]
    let intervals = vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]];
    let queries = vec![2, 3, 4, 5];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![3, 3, 1, 4]);
}

#[test]
fn test_example_2() {
    // Input: intervals = [[2,3],[2,5],[1,8],[20,25]], queries = [2,19,5,22]
    // Output: [2,-1,4,6]
    let intervals = vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]];
    let queries = vec![2, 19, 5, 22];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![2, -1, 4, 6]);
}

#[test]
fn test_no_matching_intervals() {
    // Test when no intervals contain the query points
    let intervals = vec![vec![1, 3], vec![5, 7], vec![9, 11]];
    let queries = vec![4, 8, 12];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![-1, -1, -1]);
}

#[test]
fn test_single_interval() {
    // Test with a single interval
    let intervals = vec![vec![1, 10]];
    let queries = vec![1, 5, 10, 11];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![10, 10, 10, -1]);
}

#[test]
fn test_multiple_containing_intervals() {
    // Test when multiple intervals contain the query point
    let intervals = vec![vec![1, 10], vec![2, 5], vec![3, 4], vec![6, 8]];
    let queries = vec![3, 7];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![2, 3]);
}

#[test]
fn test_edge_cases() {
    // Test edge cases with intervals at the boundaries
    let intervals = vec![vec![1, 1], vec![10, 10]];
    let queries = vec![1, 5, 10];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![1, -1, 1]);
}

#[test]
fn test_large_intervals() {
    // Test with large intervals
    let intervals = vec![vec![1, 1000000], vec![500000, 600000]];
    let queries = vec![500000, 600001];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![100001, 1000000]);
}

#[test]
fn test_overlapping_intervals() {
    // Test with many overlapping intervals
    let intervals = vec![
        vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9]
    ];
    let queries = vec![3, 6];
    assert_eq!(Solution::minimum_interval_size(intervals, queries), vec![5, 5]);
}
